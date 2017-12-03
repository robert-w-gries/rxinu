mod cooperative_scheduler;
pub mod process;
pub mod process_id;
pub mod process_list;

use self::cooperative_scheduler as scheduler;
use spin::Mutex;
use syscall::error::Error;

pub use self::process::{Process, State};
pub use self::process_id::ProcessId;
pub use self::process_list::ProcessList;
pub use self::scheduler::Scheduler;

pub trait DoesScheduling {
    fn create(&mut self, func: extern fn()) -> Result<Process, Error>;
    fn getid(&self) -> &ProcessId;
    fn ready(&mut self, id: ProcessId);
    fn resched(&mut self);
}

const MAX_PROCS: usize = usize::max_value() -1;
// TODO: Use the proper stack size
//const INIT_STK_SIZE: usize = 65536;
const INIT_STK_SIZE: usize = 10000;

lazy_static! {
    pub static ref SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());
}
