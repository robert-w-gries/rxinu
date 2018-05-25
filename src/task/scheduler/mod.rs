use alloc::String;
use syscall::error::Error;
use task::process::ProcessId;

mod cooperative;

pub type GlobalScheduler = cooperative::Cooperative;

lazy_static! {
    pub static ref SCHEDULER: GlobalScheduler = GlobalScheduler::new();
}

pub trait Scheduling {
    fn create(&self, func: extern "C" fn(), name: String) -> Result<ProcessId, Error>;
    fn getid(&self) -> ProcessId;
    fn kill(&self, id: ProcessId);
    fn ready(&self, id: ProcessId);
    unsafe fn resched(&self);
}
