use alloc::{String, Vec, VecDeque};
use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering};
use spin::RwLock;
use syscall::error::Error;
use task::{process, Process, ProcessId, ProcessList, Scheduling, State, INIT_STK_SIZE};

pub type Scheduler = Cooperative;

pub struct Cooperative {
    current_pid: AtomicUsize,
    inner: RwLock<CooperativeInner>,
}

struct CooperativeInner {
    proc_table: ProcessList,
    ready_list: VecDeque<ProcessId>,
}

impl Scheduling for Cooperative {
    fn create(&self, new_proc: extern "C" fn(), name: String) -> Result<ProcessId, Error> {
        let mut stack: Vec<usize> = vec![0; INIT_STK_SIZE];

        let stack_values: Vec<usize> = vec![new_proc as usize, process::process_ret as usize];

        // Reserve blocks in the stack for scheduler data
        // len-1: process return instruction pointer
        // len-2: process instruction pointer (process stack pointer starts here and grows down)
        let proc_top: usize = stack.len() - stack_values.len();
        let proc_stack_pointer: usize =
            stack.as_ptr() as usize + (proc_top * mem::size_of::<usize>());

        for (i, val) in stack_values.iter().enumerate() {
            stack[proc_top + i] = *val;
        }

        let mut inner = self.inner.write();

        let proc = inner.proc_table.add()?;
        {
            proc.name = name;

            proc.context
                .set_page_table(unsafe { ::x86::shared::control_regs::cr3() as usize });

            proc.context.set_base_pointer(
                stack.as_ptr() as usize + (stack.len() * mem::size_of::<usize>()),
            );
            proc.context.set_stack(proc_stack_pointer);

            proc.kstack = Some(stack);

            Ok(proc.pid)
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
            let mut inner = self.inner.write();

            let proc = inner
                .proc_table
                .get_mut(id)
                .expect("Could not find process to kill");

            proc.set_state(State::Free);
            proc.kstack = None;
            drop(&mut proc.name);
        }

        unsafe {
            self.resched();
        }
    }

    fn ready(&self, id: ProcessId) {
        self.inner.write().ready_list.push_back(id);
    }

    /// Safety: This method will deadlock if any scheduling locks are still held
    unsafe fn resched(&self) {
        // Important: Ensure lock is dropped before context switch
        let mut inner = self.inner.write();

        let curr_id: ProcessId = self.getid();
        let next_id = if let Some(next_id) = inner.ready_list.pop_front() {
            assert!(curr_id != next_id);
            next_id
        } else {
            return;
        };

        // Add current process back to ready list
        let current_proc_ptr = {
            let mut current_proc = inner
                .proc_table
                .get_mut(curr_id)
                .expect("Could not find current process in process table");

            // Add current process back to ready list
            if current_proc.state == State::Current {
                current_proc.set_state(State::Ready);
            }

            current_proc as *mut Process
        };

        let next_proc_ptr = {
            let mut next_proc = inner
                .proc_table
                .get_mut(next_id)
                .expect("Process ID in ready list is not in process table");

            assert!(next_proc.kstack.is_some());

            next_proc.set_state(State::Current);

            next_proc as *mut Process
        };

        if (*current_proc_ptr).state == State::Ready {
            inner.ready_list.push_back(curr_id);
        }

        self.current_pid.store(next_id.0, Ordering::SeqCst);

        // Drop locks to prevent deadlock after context switch
        drop(inner);

        (*current_proc_ptr)
            .context
            .switch_to(&mut (*next_proc_ptr).context);
    }
}

impl Cooperative {
    pub fn new() -> Cooperative {
        Cooperative {
            current_pid: AtomicUsize::new(ProcessId::NULL_PROCESS.get_usize()),
            inner: RwLock::new(CooperativeInner {
                proc_table: ProcessList::new(),
                ready_list: VecDeque::<ProcessId>::new(),
            }),
        }
    }
}
