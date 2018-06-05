pub mod process;
pub mod scheduler;

pub use self::process::{Process, ProcessId, ProcessRef, ProcessTable, State};

const MAX_PID: usize = <usize>::max_value() - 1;
