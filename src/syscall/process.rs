use alloc::String;
use syscall::error::Error;
use task::scheduler::{global_sched, Scheduling};
use task::ProcessId;

/// Wrapper around scheduler.create() and ready() that can be called in processes
pub fn create(name: String, prio: usize, proc_entry: extern "C" fn()) -> Result<ProcessId, Error> {
    let pid = global_sched()
        .create(name, prio, proc_entry)
        .expect("Could not create new process!");
    global_sched().ready(pid)?;
    Ok(pid)
}

/// Wrapper around scheduler.kill()
pub fn kill(proc_id: ProcessId) -> Result<(), Error> {
    global_sched().kill(proc_id)?;
    Ok(())
}
