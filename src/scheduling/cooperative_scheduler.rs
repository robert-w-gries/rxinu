use alloc::{BTreeMap, VecDeque};
use core::cmp::Ord;
use core::mem;
use core::sync::atomic::Ordering;
use scheduling::{Process, ProcessId, ProcessList, State, Scheduler, INIT_STK_SIZE};
use spin::Mutex;
use syscall::error::Error;

pub static SCHEDULER: CoopScheduler = CoopScheduler::new();

struct CoopScheduler {
    current_pid: ProcessId,
    proc_table: Mutex<ProcessList<BTreeMap<ProcessId, Process>>>,
    ready_list: Mutex<VecDeque<ProcessId>>,
}

impl Scheduler for CoopScheduler {
    fn current_process(&self) -> Option<Process> {
        self.proc_table.lock().get(self.getid())
    }

    fn create(&self, func: extern fn()) -> Result<ProcessId, Error> {
        use arch::context::Context;
        use arch::memory::paging;

        let mut process: Process = self.proc_table.lock().add()?;

        let mut stack = vec![0; INIT_STK_SIZE].into_boxed_slice();

        let offset = stack.len() - mem::size_of::<usize>();
        unsafe {
            let func_ptr = stack.as_mut_ptr().offset(offset as isize);
            *(func_ptr as *mut usize) = func as usize;
        }

        process.context().set_page_table(unsafe { paging::ActivePageTable::new().address() });
        process.context().set_stack(stack.as_ptr() as usize + offset);

        Ok(process.pid())
    }

    fn getid(&self) -> ProcessId {
        self.current_pid.0.load(Ordering::SeqCst)
    }

    fn ready(&self, id: ProcessId) {
        self.ready_list.lock().push_back(id);
    }

    fn resched(&self) {
        if let Some(next_id) = self.ready_list.lock().pop_front() {
            let current: Process = self.current_process().expect("No current process available");

            let next: Process = self.proc_table.lock().get(next_id).expect("Could not find new process");
            next.state = State::Current;

            self.current_process().context().switch_to(next.context());
            self.setid(next.pid());
        }
    }

    fn setid(&self, id: ProcessId) {
        self.current_pid.0.store(id.0, Ordering::SeqCst);
    }
}

impl CoopScheduler {
    fn new() -> CoopScheduler {
        CoopScheduler {
            current_pid: ProcessId(AtomicUsize::new(0)),
            proc_table: Mutex::new(ProcessList::new()),
            ready_list: Mutex::new(VecDeque::<ProcessId>::new()),
        }
    }
}
