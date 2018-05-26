use alloc::String;
use task::ProcessId;
use task::scheduler::{Scheduling, SCHEDULER};

/// Wrapper around scheduler.create() and ready() that can be called in processes
pub fn create(name: String, prio: usize, proc_entry: extern "C" fn()) -> ProcessId {
    use arch::interrupts;

    interrupts::disable_then_restore(|| {
        let pid = SCHEDULER
            .create(name, prio, proc_entry)
            .expect("Could not create new process!");
        SCHEDULER.ready(pid.clone());
        pid
    })
}
