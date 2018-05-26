pub mod process;
pub mod process_list;
pub mod scheduler;

pub use self::process::{Process, ProcessId, State};
pub use self::process_list::ProcessList;

const MAX_PROCS: usize = usize::max_value() - 1;
