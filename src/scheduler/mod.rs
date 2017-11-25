mod process;

trait Scheduler {
    fn context_switch(&self);
    fn current(&self) -> Option<Process>;
    fn create(&self, func: extern fn(), priority: Priority) -> Result<Process>;
    fn getid(&self);
    fn ready(&self);
    fn resched(&self);
}

static PROC_TABLE: Mutex<ProcessList> = Mutex::new(ProcessList::new());
static CURRENT_PID: Mutex<ProcessId> = ProcessId::MISSING;

const MAX_PROCS: usize = usize:max_value() -1;
