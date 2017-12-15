use alloc::{String, Vec, VecDeque};
use core::mem;
use core::ops::DerefMut;
use core::sync::atomic::{AtomicUsize, Ordering};
use scheduling::{DoesScheduling, Process, ProcessId, ProcessList, State, INIT_STK_SIZE};
use scheduling::process;
use spin::RwLock;
use syscall::error::Error;

pub type Scheduler = CoopScheduler;

#[derive(Debug)]
pub struct CoopScheduler {
    current_pid: AtomicUsize,
    proc_table: RwLock<ProcessList>,
    ready_list: RwLock<VecDeque<ProcessId>>,
}

impl DoesScheduling for CoopScheduler {
    fn create(&self, new_proc: extern "C" fn(), name: String) -> Result<ProcessId, Error> {
        use arch::memory::paging;

        let mut stack: Vec<usize> = vec![0; INIT_STK_SIZE];

        // Reserve 3 blocks in the stack for scheduler data
        // Stack order (top -> bottom)
        // len-1: pointer to scheduler object (self) is used in process return
        // len-2: process return instruction pointer
        // len-3: process instruction pointer (process stack pointer starts here and grows down)
        let proc_top: usize = stack.len() - 3;
        let proc_stack_pointer: usize =
            stack.as_ptr() as usize + (proc_top * mem::size_of::<usize>());

        let stack_values: Vec<usize> = vec![
            new_proc as usize,
            process::process_ret as usize,
            self as *const Scheduler as usize,
        ];

        for (i, val) in stack_values.iter().enumerate() {
            stack[proc_top + i] = *val;
        }

        let mut proc_table_lock = self.proc_table.write();

        let process_lock = proc_table_lock.add()?;
        {
            let mut process = process_lock.write();

            process.kstack = Some(stack);
            process.name = name;

            process
                .context
                .set_page_table(unsafe { paging::ActivePageTable::new().address() });

            process.context.set_stack(proc_stack_pointer);

            Ok(process.pid)
        }
    }

    /// Get current process id
    fn getid(&self) -> ProcessId {
        ProcessId(self.current_pid.load(Ordering::SeqCst))
    }

    /// Scheduler's method to kill processes
    /// Currently, we just mark the process as FREE and leave its memory in the proc table
    fn kill(&self, id: ProcessId) {
        // We need to scope the manipulation of the process so we don't deadlock in resched()
        {
            let proc_table_lock = self.proc_table.read();
            let mut proc_lock = proc_table_lock
                .get(id)
                .expect("Could not find process to kill")
                .write();

            proc_lock.set_state(State::Free);
            proc_lock.kstack = None;
            drop(&mut proc_lock.name);
        }

        unsafe {
            self.resched();
        }
    }

    fn ready(&self, id: ProcessId) {
        self.ready_list.write().push_back(id);
    }

    /// Safety: This method will deadlock if any scheduling locks are still held
    unsafe fn resched(&self) {
        // skip expensive locks if possible
        if self.ready_list.read().is_empty() {
            return;
        }

        // TODO: Investigate less hacky way of context switching without deadlocking
        let mut prev_ptr = 0 as *mut Process;
        let mut next_ptr = 0 as *mut Process;

        // Separate the locks from the context switch through scoping
        // This will avoid deadlocks on next resched() call
        {
            let proc_table_lock = self.proc_table.read();
            let mut ready_list_lock = self.ready_list.write();

            let curr_id: ProcessId = self.getid();

            let mut prev = proc_table_lock
                .get(curr_id)
                .expect("Could not find previous process")
                .write();

            // we want to be able to return to this process later
            if prev.state == State::Current {
                prev.set_state(State::Ready);
                ready_list_lock.push_back(curr_id);
            }

            if let Some(next_id) = ready_list_lock.pop_front() {
                let mut next = proc_table_lock
                    .get(next_id)
                    .expect("Could not find new process")
                    .write();

                next.set_state(State::Current);

                self.current_pid
                    .store(next.pid.get_usize(), Ordering::SeqCst);

                // Save process pointers since context switch is out of scope
                prev_ptr = prev.deref_mut() as *mut Process;
                next_ptr = next.deref_mut() as *mut Process;
            }
        }

        if next_ptr as usize != 0 {
            assert!(
                prev_ptr as usize != 0,
                "Pointer to previous process has not been set!"
            );
            let prev: &mut Process = &mut *prev_ptr;
            let next: &mut Process = &mut *next_ptr;
            prev.context.switch_to(&mut next.context);
        }
    }
}

impl CoopScheduler {
    pub fn new() -> CoopScheduler {
        CoopScheduler {
            current_pid: AtomicUsize::new(ProcessId::NULL_PROCESS.get_usize()),
            proc_table: RwLock::new(ProcessList::new()),
            ready_list: RwLock::new(VecDeque::<ProcessId>::new()),
        }
    }
}
