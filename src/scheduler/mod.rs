mod process;

trait Scheduler {
    fn context_switch(&self);
    fn current(&self) -> Option<Process>;
    fn create(&self, func: extern fn()) -> Result<Process>;
    fn getid(&self);
    fn ready(&self);
    fn resched(&self);
}

const MAX_PROCS: u64 = u64:max_value() -1;
