use alloc::{BTreeMap, VecDeque};
use scheduling::{Process, ProcessList, INIT_STK_SIZE};

pub static SCHEDULER: CoopScheduler = CoopScheduler::new();

struct CoopScheduler {
    current_pid: ProcessId,
    proc_table: Mutex<ProcessList<BTreeMap>>,
    ready_list: Mutex<VecDeque<ProcessId>>,
}

impl Scheduler for CoopScheduler {
    fn context_switch(&self, next_process: Process) {
        // self.current_process().context.switch_to(next_process.context);
    }

    pub fn current_process(&self) -> Option<Process> {
        self.proc_table.lock().get(self.getid())
    }

    pub fn create(&self, func: extern fn()) -> Result<Process> {
        use arch::context::Context;
        use arch::memory::paging;

        let mut proc: Process = self.proc_table.lock().add()?;

        let mut stack = vec![0; INIT_STK_SIZE].into_boxed_slice();

        let offset = stack.len() - mem::size_of::<usize>();
        unsafe {
            let func_ptr = stack.as_mut_ptr().offset(offset as isize);
            *(func_ptr as *mut usize) = func as usize;
        }

        proc.context.set_page_table(unsafe { paging::ActivePageTable::new().address() });
        proc.context.set_stack(stack.as_ptr() as usize + offset);

        Ok(proc.pid)
    }

    pub fn getid(&self) -> ProcessId {
        self.current_pid.load(SeqCst)
    }

    pub fn ready(&self, id: ProcessId) {
        ready_list.push_back(id);
    }

    pub fn resched(&self) {
        if let Some(id) = self.ready_list.pop_front() {
            let current: Process = self.current_process();

            let next: Process = self.proc_table.lock().get(next_id);
            next.state = State::Current;

            self.current_process().context.switch_to(next.context);
            self.setid(next.pid);
        }
    }

    fn setid(&self, id: ProcessId) {
        self.current_pid.store(id.0, SeqCst);
    }
}

impl CoopScheduler {
    fn new() -> CoopScheduler {
        CoopScheduler {
            current_pid: Mutex::new(ProcessId(0)),
            proc_table: Mutex::new(ProcessList::new()),
            ready_list: Mutex::new(VecDeque<ProcessId>),
        }
    }
}
