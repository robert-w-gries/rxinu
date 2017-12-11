use scheduling::{DoesScheduling, ProcessId, SCHEDULER};

/// Wrapper around scheduler.create() and ready() that can be called in processes
pub fn create(new_proc: extern "C" fn()) -> ProcessId {
    let pid = SCHEDULER.create(new_proc).expect("Could not create new process!");
    SCHEDULER.ready(pid.clone());
    pid
}
