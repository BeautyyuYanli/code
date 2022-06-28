use nix::sys::ptrace;
use nix::sys::signal;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::Pid;
use std::collections::HashMap;
use std::os::unix::prelude::CommandExt;
use std::process::Child;
use std::process::Command;
use std::mem::size_of;

#[derive(Debug, Clone)]
pub enum Status {
    /// Initial status of the inferior process.
    Start,

    /// Indicates inferior stopped. Contains the signal that stopped the process, as well as the
    /// current instruction pointer that it is stopped at.
    Stopped(signal::Signal, usize),

    /// Stopped at a breakpoint. Contains the current instruction pointer that it is stopped at.
    StoppedAfterBreakpoint(usize),

    /// Indicates inferior exited normally. Contains the exit status code.
    Exited(i32),

    /// Indicates the inferior exited due to a signal. Contains the signal that killed the
    /// process.
    Signaled(signal::Signal),
}

pub fn align_addr_to_word(addr: usize) -> usize {
    addr & (-(size_of::<usize>() as isize) as usize)
}

/// This function calls ptrace with PTRACE_TRACEME to enable debugging on a process. You should use
/// pre_exec with Command to call this in the child process.
fn child_traceme() -> Result<(), std::io::Error> {
    ptrace::traceme().or(Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "ptrace TRACEME failed",
    )))
}

pub struct Inferior {
    child: Child,
    pub breakpoints: Option<HashMap<usize, u8>>,
}

impl Inferior {
    fn write_byte(&self, addr: usize, val: u8) -> Result<u8, nix::Error> {
        let aligned_addr = align_addr_to_word(addr);
        let byte_offset = addr - aligned_addr;
        let word = ptrace::read(self.pid(), aligned_addr as ptrace::AddressType)? as u64;
        let orig_byte = (word >> 8 * byte_offset) & 0xff;
        let masked_word = word & !(0xff << 8 * byte_offset);
        let updated_word = masked_word | ((val as u64) << 8 * byte_offset);
        ptrace::write(
            self.pid(),
            aligned_addr as ptrace::AddressType,
            updated_word as *mut std::ffi::c_void,
        )?;
        Ok(orig_byte as u8)
    }

    /// Returns the pid of this inferior.
    pub fn pid(&self) -> Pid {
        nix::unistd::Pid::from_raw(self.child.id() as i32)
    }

    /// Attempts to start a new inferior process. Returns Some(Inferior) if successful, or None if
    /// an error is encountered.
    pub fn new(target: &str, args: &Vec<String>, breakpoints_tmp: &HashMap<usize, u8>) -> Option<Inferior> {
        let mut command = Command::new(target);
        command.args(args);
        unsafe { command.pre_exec(child_traceme); }
        let mut inferior = Inferior { 
            child: command.spawn().ok()?, 
            breakpoints: Some(breakpoints_tmp.clone()),
        };
        match inferior.wait(None).ok()? {
            Status::Stopped(sig, _) => match sig {
                signal::Signal::SIGTRAP => {
                    for (address, _) in breakpoints_tmp {
                        inferior.breakpoint_inject(*address);
                    }
                    Some(inferior)
                },
                _ => None
            },
            _ => None
        }
    }


    /// Calls waitpid on this inferior and returns a Status to indicate the state of the process
    /// after the waitpid call.
    pub fn wait(&self, options: Option<WaitPidFlag>) -> Result<Status, nix::Error> {
        Ok (match waitpid(self.pid(), options)? {
            WaitStatus::Exited(_pid, exit_code) => Status::Exited(exit_code),
            WaitStatus::Signaled(_pid, signal, _core_dumped) => Status::Signaled(signal),
            WaitStatus::Stopped(_pid, signal) => {
                let regs = ptrace::getregs(self.pid())?;
                match signal {
                    signal::Signal::SIGTRAP => {
                        if self.breakpoints.as_ref().unwrap().contains_key(&((regs.rip - 1) as usize)) {
                            Status::StoppedAfterBreakpoint(regs.rip as usize)
                        }
                        else {
                            Status::Stopped(signal, regs.rip as usize)
                        }
                    }
                    _ => Status::Stopped(signal, regs.rip as usize)
                }
            }
            other => panic!("waitpid returned unexpected status: {:?}", other),
        })
    }
     
    /// Continue the stopped inferior process, then wait for a result.
    pub fn continues(&mut self, recent_status: &Status) -> Result<Status, nix::Error> {
        if let Status::StoppedAfterBreakpoint(rip) = recent_status {
            // restore the original byte at the breakpoint
            self.write_byte(
                rip - 1, 
                *self.breakpoints.as_ref().unwrap().get(&(rip - 1 as usize)).unwrap()
            )?;
            // rewind the instruction pointer to the last instruction
            let mut regs = ptrace::getregs(self.pid())?;
            regs.rip = (rip - 1) as u64;
            ptrace::setregs(self.pid(), regs)?;
            // go by one step
            ptrace::step(self.pid(), None)?;
            let step_status = self.wait(None)?;
            match step_status{
                Status::StoppedAfterBreakpoint(rip_step) => {
                    if rip_step == *rip {
                        // restore the breakpoint
                        self.write_byte(rip - 1, 0xcc).expect("Injecting breakpoint: Unexpected Error!");
                    }
                    else { return Ok(step_status); }
                },
                _ => { return Ok(step_status); }
            }
        }
        ptrace::cont(self.pid(), None)?;
        self.wait(None)
    }
    
    /// Kill a process, then wait for a result.
    pub fn kill(&mut self) -> Result<Status, nix::Error>{
        // assume the error from kill
        self.child.kill().expect("Killing subprocess: Unexpected Error!");
        self.wait(None)
    }
    
    /// Get the next backtrace (rip, rbp) with respect to the given last_rbp.
    /// If last_rbp is None, then the first backtrace (rip, rbp) is returned.
    pub fn backtrace(&self, last_rbp: Option<usize>) -> (usize, usize){
        match last_rbp{
            None => {
                let regs = ptrace::getregs(self.pid())
                    .expect("Getting backtrace: Unexpected Error by first frame!");
                ( regs.rip as usize, regs.rbp as usize )
            },
            Some(last_rbp) => {
                (
                    ptrace::read(self.pid(), (last_rbp + 8) as ptrace::AddressType)
                        .expect("Getting backtrace: Unexpected Error by reading next rip!") as usize,
                    ptrace::read(self.pid(), last_rbp as ptrace::AddressType)
                        .expect("Getting backtrace: Unexpected Error by reading next rbp!") as usize
                )
            }
        }
    }
    
    pub fn breakpoint_inject(&mut self, address: usize){
        let original_byte = self.write_byte(address, 0xcc).expect("Injecting breakpoint: Unexpected Error!");
        match self.breakpoints.as_mut().unwrap().get_mut(&address) {
            Some(original_byte_mut) => { *original_byte_mut = original_byte; }
            None => { self.breakpoints.as_mut().unwrap().insert(address, original_byte); }
        }
    }
}