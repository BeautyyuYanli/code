use std::collections::HashMap;

use crate::debugger_command::DebuggerCommand;
use crate::inferior::{Inferior, self};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use crate::dwarf_data::{DwarfData, Error as DwarfError};

pub struct Debugger {
    target: String,
    history_path: String,
    readline: Editor<()>,
    inferior: Option<Inferior>,
    debug_data: DwarfData,
    breakpoints_tmp: Option<HashMap<usize, u8>>,
    recent_status: Option<inferior::Status>,
}

fn parse_address(addr: &str) -> Option<usize> {
    let addr_without_0x = if addr.to_lowercase().starts_with("0x") {
        &addr[2..]
    } else {
        &addr
    };
    usize::from_str_radix(addr_without_0x, 16).ok()
}

impl Debugger {
    /// Initializes the debugger.
    pub fn new(target: &str) -> Debugger {
        let debug_data = match DwarfData::from_file(target) {
            Ok(val) => val,
            Err(DwarfError::ErrorOpeningFile) => {
                println!("Could not open file {}", target);
                std::process::exit(1);
            }
            Err(DwarfError::DwarfFormatError(err)) => {
                println!("Could not debugging symbols from {}: {:?}", target, err);
                std::process::exit(1);
            }
        };
        // debug_data.print();
        let history_path = format!("{}/.deet_history", std::env::var("HOME").unwrap());
        let mut readline = Editor::<()>::new();
        // Attempt to load history from ~/.deet_history if it exists
        let _ = readline.load_history(&history_path);

        Debugger {
            target: target.to_string(),
            history_path,
            readline,
            inferior: None,
            debug_data,
            breakpoints_tmp: Some(HashMap::new()),
            recent_status: Some(inferior::Status::Start)
        }
    }

    /// Print the given address and check if it's `main` function. If it is, return true.
    fn address_process(&self, address: usize) -> bool{
        let line = self.debug_data.get_line_from_addr(address);
        let function = self.debug_data.get_function_from_addr(address);
        match (line, function) {
            (Some(line), Some(function)) => {
                println!("{} ({})", line, function);
                function == "main"
            }
            _ => {
                panic!("Address can't be parsed: {}", address);
            }
        }
    }
    
    /// Print the status of the inferior process. Modify the `inferior` field of the debugger to None if needed.
    fn result_process(&mut self, result: Result<inferior::Status, nix::Error>) {
        println!("======================================================");
        match result {
            Ok(status) => {
                println!("Command finished. Subprocess status: {:?}", &status);
                match &status {
                    inferior::Status::Stopped(_signal, rip) => {
                        println!("Stopped at:");
                        self.address_process(*rip);
                    },
                    inferior::Status::StoppedAfterBreakpoint(rip) => {
                        println!("Stopped at breakpoint:");
                        self.address_process(rip - 1);
                    },
                    inferior::Status::Exited(_exit_code)  => {
                        self.breakpoints_tmp = Some(HashMap::new());
                        self.recent_status = None;
                        self.inferior = None;
                    },
                    inferior::Status::Signaled(_signal) => {
                        self.breakpoints_tmp = Some(HashMap::new());
                        self.recent_status = None;
                        self.inferior = None;
                    },
                    _ => {}
                }
                self.recent_status = Some(status);
            }
            Err(error) => {
                println!("Command failed: {:?}", error);
            }
        };
    }

    pub fn run(&mut self) {
        loop {
            match self.get_next_command() {
                DebuggerCommand::Error(error) => {
                    println!("{}", error);
                }
                DebuggerCommand::Quit => { 
                    match &self.inferior {
                        Some(_) => {
                            println!("Command failed: A subprocess exists. Type `k` to kill.");
                        }
                        None => {return;}
                    }
                },
                DebuggerCommand::Run(args) => {
                    match &mut self.inferior {
                        Some(_) => {
                            println!("Command failed: A subprocess exists. Type `k` to kill.");
                        },
                        None => {
                            // Create the inferior
                            if let Some(inferior) = Inferior::new(&self.target, &args, self.breakpoints_tmp.as_ref().unwrap()) {
                                self.breakpoints_tmp = None;
                                self.inferior = Some(inferior);
                                let result = self.inferior.as_mut().unwrap().continues(&inferior::Status::Start);
                                self.result_process(result);
                            } else {
                                println!("Command failed: Error starting subproces.");
                            }
                        }
                    }

                },
                DebuggerCommand::Continue => {
                    match &mut self.inferior {
                        Some(inferior) => {
                            let result = inferior.continues(&self.recent_status.as_ref().unwrap());
                            self.result_process(result);
                        },
                        None => {
                            println!("Command failed: No subprocess exists.")
                        }
                    }
                },
                DebuggerCommand::Kill => {
                    match &mut self.inferior {
                        Some(inferior) => {
                            let result = inferior.kill();
                            self.result_process(result);
                        },
                        None => {
                            println!("Command failed: No subprocess exists.")
                        }
                    }
                },
                DebuggerCommand::Backtrace => {
                    match &self.inferior {
                        Some(inferior) => {
                            let (mut rip, mut rbp) = inferior.backtrace(None);
                            while !self.address_process(rip) {
                                (rip, rbp) = inferior.backtrace(Some(rbp));
                            }
                        },
                        None => {
                            println!("Command failed: No subprocess exists.")
                        }
                    }
                },
                DebuggerCommand::Break(arg) => {
                    fn add_breakpoint (debugger : &mut Debugger, mut address: usize) {
                        address = inferior::align_addr_to_word(address);
                        match &mut debugger.inferior {
                            Some(inferior) => inferior.breakpoint_inject(address),
                            None => {debugger.breakpoints_tmp.as_mut().unwrap().insert(address, 0xcc);}
                        }
                        debugger.address_process(address);
                    }
                    if &arg[0..1] == "*" {
                        let address = parse_address(&arg[1..]);
                        if let Some(address) = address {
                            add_breakpoint(self, address);
                        } else {
                            println!("Command failed: Invalid address.");
                        }
                    }
                    else if let Ok(line) = arg[0..].parse::<usize>(){
                        let address = self.debug_data.get_addr_for_line(Some(&format!("{}.c", &self.target)), line);
                        if let Some(address) = address {
                            add_breakpoint(self, address);
                        } else {
                            println!("Command failed: Invalid line.");
                        }
                    } 
                    else {
                        let address = self.debug_data.get_addr_for_function(Some(&format!("{}.c", &self.target)), &arg);
                        if let Some(address) = address {
                            add_breakpoint(self, address);
                        } else {
                            println!("Command failed: Can't parse breakpoint.");
                        }
                    }
                },
            }
        }
    }

    /// This function prompts the user to enter a command, and continues re-prompting until the user
    /// enters a valid command. It uses DebuggerCommand::from_tokens to do the command parsing.
    ///
    /// You don't need to read, understand, or modify this function.
    fn get_next_command(&mut self) -> DebuggerCommand {
        loop {
            // Print prompt and get next line of user input
            match self.readline.readline("\n(deet) ") {
                Err(ReadlineError::Interrupted) => {
                    // User pressed ctrl+c. We're going to ignore it
                    println!("Type \"quit\" to exit");
                }
                Err(ReadlineError::Eof) => {
                    // User pressed ctrl+d, which is the equivalent of "quit" for our purposes
                    return DebuggerCommand::Quit;
                }
                Err(err) => {
                    panic!("Unexpected I/O error: {:?}", err);
                }
                Ok(line) => {
                    if line.trim().len() == 0 {
                        continue;
                    }
                    self.readline.add_history_entry(line.as_str());
                    if let Err(err) = self.readline.save_history(&self.history_path) {
                        println!(
                            "Warning: failed to save history file at {}: {}",
                            self.history_path, err
                        );
                    }
                    let tokens: Vec<&str> = line.split_whitespace().collect();
                    if let Some(cmd) = DebuggerCommand::from_tokens(&tokens) {
                        return cmd;
                    } else {
                        println!("Unrecognized command.");
                    }
                }
            }
        }
    }
}
