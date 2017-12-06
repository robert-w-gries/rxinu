mod cooperative_scheduler;
pub mod process;
pub mod process_id;
pub mod process_list;

use self::cooperative_scheduler as scheduler;
use spin::Mutex;
use syscall::error::Error;
use spin::Once;

pub use self::process::{Process, State};
pub use self::process_id::ProcessId;
pub use self::process_list::ProcessList;
pub use self::scheduler::Scheduler;

pub trait DoesScheduling {
    fn create(&self, func: extern fn()) -> Result<Process, Error>;
    fn getid(&self) -> &ProcessId;
    fn kill(&self, id: ProcessId);
    fn ready(&self, id: ProcessId);
    fn resched(&self);
}

const MAX_PROCS: usize = usize::max_value() -1;
// TODO: Use the proper stack size
//const INIT_STK_SIZE: usize = 65536;
const INIT_STK_SIZE: usize = 10000;

//lazy_static! {
//    pub static ref SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());
//}
static SCHEDULER: Once<Scheduler> = Once::new();

pub fn scheduler() -> &'static Scheduler {
    SCHEDULER.call_once(|| Scheduler::new())
}
