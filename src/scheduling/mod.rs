mod cooperative_scheduler;
pub mod process;

use self::cooperative_scheduler as scheduler;
use syscall::error::Error;

pub use self::process::{Process, ProcessId, ProcessList, State};
pub use self::scheduler::SCHEDULER;

trait Scheduler {
    fn current_process(&self) -> Option<Process>;
    fn create(&self, func: extern fn()) -> Result<ProcessId, Error>;
    fn getid(&self) -> ProcessId;
    fn ready(&self, id: ProcessId);
    fn resched(&self);
    fn setid(&self, id: ProcessId);
}

const MAX_PROCS: u64 = u64::max_value() -1;
const INIT_STK_SIZE: usize = 65536;
