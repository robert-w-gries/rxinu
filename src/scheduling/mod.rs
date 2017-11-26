mod process;

trait Scheduler {
    fn context_switch(&self, next_process: Process);
    fn current_process(&self) -> Option<Process>;
    fn create(&self, func: extern fn()) -> Result<Process>;
    fn getid(&self) -> ProcessId;
    fn ready(&self, id: ProcessId);
    fn resched(&self);
}

const MAX_PROCS: u64 = u64::max_value() -1;
