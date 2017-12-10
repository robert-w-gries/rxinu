mod cooperative_scheduler;
pub mod process;
pub mod process_list;

use self::cooperative_scheduler as scheduler;
use syscall::error::Error;

pub use self::process::{Process, ProcessId, State};
pub use self::process_list::ProcessList;
pub use self::scheduler::Scheduler;

pub trait DoesScheduling {
    fn create(&self, func: extern "C" fn()) -> Result<ProcessId, Error>;
    fn getid(&self) -> ProcessId;
    fn kill(&self, id: ProcessId);
    fn ready(&self, id: ProcessId);
    unsafe fn resched(&self);
}

const MAX_PROCS: usize = usize::max_value() - 1;
// TODO: Use the proper stack size
//const INIT_STK_SIZE: usize = 65536;
const INIT_STK_SIZE: usize = 1000;

lazy_static! {
    pub static ref SCHEDULER: Scheduler = Scheduler::new();
}
