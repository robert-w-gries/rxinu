use alloc::String;
use spin::Once;
use syscall::error::Error;
use task::process::{ProcessId, ProcessRef};

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
