use alloc::String;
use syscall::error::Error;
use task::ProcessId;
use task::scheduler::{Scheduling, global_sched};

/// Wrapper around scheduler.create() and ready() that can be called in processes
pub fn create(name: String, prio: usize, proc_entry: extern "C" fn()) -> Result<ProcessId, Error> {
    let pid = global_sched()
        .create(name, prio, proc_entry)
        .expect("Could not create new process!");
    global_sched().ready(pid)?;
    Ok(pid)
}
