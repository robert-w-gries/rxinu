use alloc::String;
use syscall::error::Error;
use task::process::ProcessId;

mod cooperative;
mod preemptive;

pub type GlobalScheduler = preemptive::Preemptive;

lazy_static! {
    pub static ref SCHEDULER: GlobalScheduler = GlobalScheduler::new();
}

pub trait Scheduling {
    fn create(&self, name: String, prio: usize, func: extern "C" fn()) -> Result<ProcessId, Error>;
    fn getid(&self) -> ProcessId;
    fn kill(&self, id: ProcessId);
    fn ready(&self, id: ProcessId);
    unsafe fn resched(&self);
    fn tick(&self);
}
