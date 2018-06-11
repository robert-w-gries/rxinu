pub mod process;
pub mod scheduler;

pub use self::process::{Process, ProcessId, ProcessRef, State};
pub use self::scheduler::{global_sched, Scheduling};

const MAX_PID: usize = <usize>::max_value() - 1;
