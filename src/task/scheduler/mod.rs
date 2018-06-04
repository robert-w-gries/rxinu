use alloc::String;
use spin::Once;
use syscall::error::Error;
use task::process::ProcessId;

mod cooperative;
mod preemptive;

pub type GlobalScheduler = preemptive::Preemptive;

static SCHEDULER: Once<GlobalScheduler> = Once::new();

pub trait Scheduling {
    fn create(&self, name: String, prio: usize, func: extern "C" fn()) -> Result<ProcessId, Error>;
    fn getid(&self) -> ProcessId;
    fn kill(&self, pid: ProcessId) -> Result<(), Error>;
    fn ready(&self, pid: ProcessId) -> Result<(), Error>;
    unsafe fn resched(&self);
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
