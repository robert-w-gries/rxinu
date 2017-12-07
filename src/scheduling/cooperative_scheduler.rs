use alloc::VecDeque;
use alloc::boxed::Box;
use core::mem;
use core::ops::{Deref, DerefMut};
use scheduling::{Process, ProcessId, ProcessList, State, DoesScheduling, INIT_STK_SIZE};
use scheduling::process;
use spin::RwLock;
use syscall::error::Error;

pub type Scheduler = CoopScheduler;

pub struct CoopScheduler {
    current_pid: ProcessId,
    proc_table: RwLock<ProcessList>,
    ready_list: RwLock<VecDeque<ProcessId>>,
}

impl DoesScheduling for CoopScheduler {
    fn create(&mut self, new_proc: extern fn()) -> Result<ProcessId, Error> {
        use arch::memory::paging;

        // TODO: Investigate proper stack representation
        let mut stack: Box<[usize]> = vec![0; INIT_STK_SIZE].into_boxed_slice();

        // TODO: Modularize stack manipulation
        let index: usize = stack.len() - 3;
        let stack_offset: usize = index * mem::size_of::<usize>();

        unsafe {
            let self_ptr = stack.as_mut_ptr().offset((stack.len() - 1) as isize);
            *(self_ptr as *mut usize) = self as *mut _ as usize;

            let ret_ptr = stack.as_mut_ptr().offset((stack.len() - 2) as isize);
            *(ret_ptr as *mut usize) = process::process_ret as usize;

            let func_ptr = stack.as_mut_ptr().offset((stack.len() - 3) as isize);
            *(func_ptr as *mut usize) = new_proc as usize;
        }

        let mut proc_table_lock = self.proc_table.write();

        let process_lock = proc_table_lock.add()?;
        {
            let mut process = process_lock.write();

            process.set_scheduler(self);
            process.context.set_page_table(unsafe { paging::ActivePageTable::new().address() });
            process.context.set_stack((stack.as_ptr() as usize) + stack_offset);

            Ok(process.pid.clone())
        }
    }

    fn getid(&self) -> &ProcessId {
        &self.current_pid
    }

    /// Scheduler's method to kill processes
    /// Currently, we just mark the process as FREE and leave its memory in the proc table
    fn kill(&mut self, id: ProcessId) {
        // TODO: free the allocated stack

        // We need to scope the manipulation of the process so we don't deadlock in resched()
        {
            let mut proc_table_lock = self.proc_table.read();
            let mut proc_lock = proc_table_lock.get(id).expect("Could not find process to kill");
            let mut killed_process = proc_lock.write();
            killed_process.set_state(State::Free);
        }
        unsafe { self.resched(); }
    }

    fn ready(&mut self, id: ProcessId) {
        self.ready_list.write().push_back(id);
    }

    /// Safety: This method will deadlock if any scheduling locks are still held
    unsafe fn resched(&mut self) {

        // TODO: Investigate less hacky way of context switching without deadlocking
        let mut old_ptr = 0 as *mut Process;
        let mut next_ptr = 0 as *mut Process;

        // Separate the locks from the context switch through scoping
        {
            let proc_table_lock = self.proc_table.read();
            let mut ready_list_lock = self.ready_list.write();

            let curr_id: ProcessId = self.getid().clone();
            let mut old = proc_table_lock.get(curr_id.clone()).expect("Could not find old process").write();

            if old.state == State::Current {
                old.set_state(State::Ready);
                ready_list_lock.push_back(curr_id);
            }

            if let Some(next_id) = ready_list_lock.pop_front() {
                if next_id != self.current_pid {
                    let mut next = proc_table_lock.get(next_id).expect("Could not find new process").write();
                    next.set_state(State::Current);

                    self.current_pid = next.pid.clone();

                    // Save process pointers for out of scope context switch
                    old_ptr  = old.deref_mut() as *mut Process;
                    next_ptr = next.deref_mut() as *mut Process;
                }
            }
        }

        if next_ptr as usize != 0 {
            assert!(old_ptr as usize != 0, "Pointer to old process has not been set!");
            (&mut *old_ptr).context.switch_to(&mut (&mut *next_ptr).context);
        }
    }
}

impl CoopScheduler {
    pub fn new() -> CoopScheduler {
        CoopScheduler {
            current_pid: ProcessId::NULL_PROCESS,
            proc_table: RwLock::new(ProcessList::new()),
            ready_list: RwLock::new(VecDeque::<ProcessId>::new()),
        }
    }
}

// TODO: impl Debug for CoopScheduler
//kprintln!("\n\n");
//
//for process in ready_list_lock.iter() {
//kprintln!("READY LIST:");
//  kprintln!("Process: {:?}", process);
//}
//
//kprintln!("\n\n");
//
//for process in proc_table_lock.iter() {
//kprintln!("PROC LIST:");
//  kprintln!("Process: {:?}", process);
//}
