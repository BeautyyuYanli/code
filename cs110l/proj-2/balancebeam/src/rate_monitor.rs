use std::sync::Arc;
// use tokio::sync::{Mutex, RwLock};
use tokio::sync::Mutex;
use std::collections::{HashMap, VecDeque};
use std::time::{SystemTime, Duration};
pub struct RateMonitor {
    interval: usize,
    limitation: usize,
    counter: Mutex<HashMap<String, usize>>,
    list: Mutex<VecDeque<(String, SystemTime)>>,
    monitor_handle: Mutex<Option<tokio::task::JoinHandle<()>>>
}
impl RateMonitor {
    pub async fn new(mut limitation: usize) -> Arc<Self>{
        limitation = match limitation {
            0 => usize::max_value(),
            _ => limitation
        };
        let arc = Arc::new(RateMonitor {
            interval: 1,
            limitation,
            counter: Mutex::new(HashMap::<String, usize>::new()),
            list: Mutex::new(VecDeque::<(String, SystemTime)>::new()),
            monitor_handle: Mutex::new(None)
        });
        *arc.monitor_handle.lock().await = Some(tokio::task::spawn( monitor(arc.clone()) ));
        arc
    }
    pub async fn try_request(&self, addr: &str) -> Result<(), ()> {
        let mut counter = self.counter.lock().await;
        let mut list = self.list.lock().await;
        match counter.get_mut(addr) {
            Some(cnt) => { 
                if *cnt >= self.limitation { Err(()) }
                else {
                    *cnt += 1;
                    list.push_back((addr.to_string(), SystemTime::now()));
                    Ok(())
                }
            },
            None => {
                counter.insert(addr.to_string(), 1);
                list.push_back((addr.to_string(), SystemTime::now()));
                Ok(())
            }
        }
    }
}

async fn monitor (rate_monitor: Arc<RateMonitor>){
    loop {
        let mut list = rate_monitor.list.lock().await;
        while !list.is_empty() && list.front().unwrap().1.elapsed().unwrap() > Duration::from_secs(60)  {
            list.pop_front();
        }
        tokio::time::sleep(Duration::from_secs(rate_monitor.interval as u64)).await;
    }
}