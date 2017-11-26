pub static SCHEDULER: CoopScheduler = CoopScheduler::new();

struct CoopScheduler {
    current_pid: Mutex<ProcessId>,
    proc_table: Mutex<ProcessList>,
    ready_list: Mutex<VecDeque<ProcessId>>,
}

impl Scheduler for CoopScheduler {
    fn context_switch(&self) {
        // current_process().switch(next_process);
    }

    fn current_process(&self) -> Option<Process> {
        self.proc_table.lock().get(self.getid())
    }

    fn create(&self, func: extern fn()) -> Result<Process> {
        let mut proc: Process = PROC_TABLE.lock().add()?;

        let mut stack = vec![0; INIT_STK_SIZE].into_boxed_slice();

        let offset = stack.len() - mem::size_of::<usize>();
        unsafe {
            let func_ptr = stack.as_mut_ptr().offset(offset as isize);
            *(func_ptr as *mut usize) = func as usize;
        }

        proc.set_page_table(unsafe { paging::ActivePageTable::new().address() });
        proc.set_stack(stack.as_ptr() as usize + offset);

        Ok(proc)
    }

    fn getid(&self) -> ProcessId {
        self.current_pid.lock().unwrap()
    }

    fn ready(&self, id: ProcessId) {
        ready_list.push_back(id);
    }

    fn resched(&self) {
        let curr_proc: ProcessId = self.getid();
        let new_proc: ProcessId = self.ready_list.lock().pop_front();
        let proc: Process = self.proc_table.lock().get(new_proc);

        context_switch()
    }
}

impl CoopScheduler {
    fn new() -> CoopScheduler {
        CoopScheduler {
            proc_table: Mutex::new(ProcessList::new()),
            ready_list: Mutex::new(VecDeque),
        }
    }
}
