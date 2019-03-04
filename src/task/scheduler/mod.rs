use crate::syscall::error::Error;
use crate::task::process::{Process, ProcessId, ProcessRef};
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::sync::Arc;
use spin::{Once, RwLock};

mod cooperative;
mod preemptive;

pub type GlobalScheduler = preemptive::Preemptive;

static SCHEDULER: Once<GlobalScheduler> = Once::new();

pub trait Scheduling {
    fn create(&self, name: String, prio: usize, func: extern "C" fn()) -> Result<ProcessId, Error>;
    fn get_pid(&self) -> ProcessId;
    fn get_process(&self, pid: ProcessId) -> Result<ProcessRef, Error>;
    fn kill(&self, pid: ProcessId) -> Result<(), Error>;
    fn modify_process<F>(&self, pid: ProcessId, modify_fn: F) -> Result<ProcessRef, Error>
    where
        F: Fn(&ProcessRef);
    fn ready(&self, pid: ProcessId) -> Result<(), Error>;
    unsafe fn resched(&self) -> Result<(), Error>;
    fn tick(&self);
    fn unready(&self, pid: ProcessId) -> Result<(), Error>;
}

pub fn global_sched() -> &'static GlobalScheduler {
    SCHEDULER.call_once(|| GlobalScheduler::new())
}

/// Safety: Scheduler lock is used. This function could cause deadlock if interrupted
pub unsafe fn init() {
    global_sched().init();
}

pub struct ProcessTable {
    pub map: BTreeMap<ProcessId, ProcessRef>,
    next_pid: usize,
}

impl ProcessTable {
    pub fn new() -> ProcessTable {
        ProcessTable {
            map: BTreeMap::new(),
            next_pid: 1,
        }
    }

    pub fn add(&mut self, proc: Process) -> Result<ProcessId, Error> {
        let pid = proc.pid;
        match self
            .map
            .insert(pid, ProcessRef(Arc::new(RwLock::new(proc))))
        {
            // PID already used
            Some(_) => Err(Error::BadPid),
            None => Ok(pid),
        }
    }

    pub fn get(&self, pid: ProcessId) -> Option<&ProcessRef> {
        self.map.get(&pid)
    }

    pub fn next_pid(&mut self) -> Result<ProcessId, Error> {
        use crate::task::MAX_PID;

        while self.map.contains_key(&ProcessId(self.next_pid)) && self.next_pid < MAX_PID {
            self.next_pid += 1;
        }

        match self.next_pid {
            MAX_PID => {
                self.next_pid = 1;
                Err(Error::TryAgain)
            }
            pid => {
                self.next_pid += 1;
                Ok(ProcessId(pid))
            }
        }
    }

    pub fn insert(&mut self, pid: ProcessId, proc: ProcessRef) -> Option<ProcessRef> {
        self.map.insert(pid, proc)
    }

    pub fn remove(&mut self, pid: ProcessId) -> Option<ProcessRef> {
        self.map.remove(&pid)
    }
}
