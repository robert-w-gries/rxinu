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
    fn create(&self, new_proc: extern fn()) -> Result<Process, Error> {
        use arch::memory::paging;

        // TODO: Investigate proper stack representation
        let mut stack: Box<[usize]> = vec![0; INIT_STK_SIZE].into_boxed_slice();

        // TODO: Investigate proper offset
        //let new_proc_index: usize = stack.len() - 2;
        //let stack_offset: usize = new_proc_index * mem::size_of::<usize>();
        let index: usize = stack.len() - 3;
        let stack_offset: usize = index * mem::size_of::<usize>();

        unsafe {
            //let self_ptr = stack.as_mut_ptr().offset((stack.len() - 1) as isize);
            //*(self_ptr as *mut usize) = self as *mut _ as usize;

//kprintln!("self ptr = 0x{:x}", (self as *mut _ as usize));
            let ret_ptr = stack.as_mut_ptr().offset((stack.len() - 2) as isize);
            *(ret_ptr as *mut usize) = process::process_ret as usize;

            let func_ptr = stack.as_mut_ptr().offset((stack.len() - 3) as isize);
            *(func_ptr as *mut usize) = new_proc as usize;
        }

        let mut proc_table_lock = self.proc_table.write();

        let process_lock = proc_table_lock.add()?;
        {
            let mut process = process_lock.write();

            process.context.set_page_table(unsafe { paging::ActivePageTable::new().address() });
            process.context.set_stack((stack.as_ptr() as usize) + stack_offset);

            Ok(process.clone())
        }
    }

    fn getid(&self) -> &ProcessId {
        &self.current_pid
    }

/// Process return
    fn kill(&self, id: ProcessId) {
kprintln!("Killing!");
        self.proc_table.write().remove(id);
kprintln!("Rescheding!");
        self.resched();
    }

    fn ready(&self, id: ProcessId) {
        self.ready_list.write().push_back(id);
    }

    fn resched(&self) {
        let proc_table_lock = self.proc_table.write();
        let mut ready_list_lock = self.ready_list.write();

        let curr_id: ProcessId = self.getid().clone();
        let mut current = proc_table_lock.get(curr_id.clone()).expect("Could not find current process").write();

        if current.state == State::Current {
            current.set_state(State::Ready);
            ready_list_lock.push_back(curr_id);
        }

        if let Some(next_id) = ready_list_lock.pop_front() {
            let mut next = proc_table_lock.get(next_id).expect("Could not find new process").write();
            next.set_state(State::Current);

            self.current_pid = next.pid.clone();

            unsafe {
                current.context.switch_to(&mut next.context);
            }
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
