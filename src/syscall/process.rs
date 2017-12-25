use alloc::String;
use scheduling::{DoesScheduling, ProcessId, SCHEDULER};

/// Wrapper around scheduler.create() and ready() that can be called in processes
pub fn create(new_proc: extern "C" fn(), name: String) -> ProcessId {
    use arch::interrupts;

    interrupts::disable_interrupts_then(|| -> ProcessId {
        let pid = SCHEDULER.
            .create(new_proc, name)
            .expect("Could not create new process!");
        SCHEDULER.ready(pid.clone());
        pid
    })
}
