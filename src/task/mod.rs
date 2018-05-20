pub mod process;
pub mod process_list;
pub mod scheduler;

use alloc::String;
use syscall::error::Error;

pub use self::process::{Process, ProcessId, State};
pub use self::process_list::ProcessList;
pub use self::scheduler::cooperative::Scheduler;

pub trait DoesScheduling {
    fn create(&self, func: extern "C" fn(), name: String) -> Result<ProcessId, Error>;
    fn getid(&self) -> ProcessId;
    fn kill(&self, id: ProcessId);
    fn ready(&self, id: ProcessId);
    unsafe fn resched(&self);
}

const MAX_PROCS: usize = usize::max_value() - 1;
const INIT_STK_SIZE: usize = 1024;

lazy_static! {
    pub static ref SCHEDULER: Scheduler = Scheduler::new();
}
