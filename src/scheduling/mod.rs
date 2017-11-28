mod cooperative_scheduler;
pub mod process;
pub mod process_id;
pub mod process_list;

use self::cooperative_scheduler as scheduler;
use syscall::error::Error;

pub use self::process::{Process, State};
pub use self::process_id::ProcessId;
pub use self::process_list::ProcessList;
pub use self::scheduler::SCHEDULER;

pub trait Scheduler {
    fn create(&mut self, func: extern fn()) -> Result<&Process, Error>;
    fn getid(&self) -> &ProcessId;
    fn ready(&mut self, id: ProcessId);
    fn resched(&self);
}

const MAX_PROCS: usize = usize::max_value() -1;
const INIT_STK_SIZE: usize = 65536;
