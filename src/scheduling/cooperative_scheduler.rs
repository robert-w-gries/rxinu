use alloc::{BTreeMap, VecDeque};
use core::mem;
use core::ops::Deref;
use scheduling::{Process, ProcessId, ProcessList, State, DoesScheduling, INIT_STK_SIZE, SCHEDULER};
use spin::RwLock;
use syscall::error::Error;

pub type Scheduler = CoopScheduler;

pub struct CoopScheduler {
    current_pid: ProcessId,
    proc_table: RwLock<ProcessList>,
    ready_list: RwLock<VecDeque<ProcessId>>,
}

impl DoesScheduling for CoopScheduler {
    fn create(&mut self, func: extern fn()) -> Result<Process, Error> {
        use arch::context::Context;
        use arch::memory::paging;

        let mut stack = vec![0; INIT_STK_SIZE].into_boxed_slice();

        let offset = stack.len() - mem::size_of::<usize>();
        unsafe {
            let func_ptr = stack.as_mut_ptr().offset(offset as isize);
            *(func_ptr as *mut usize) = func as usize;
        }

        let mut proc_table_lock = self.proc_table.write();

        let process_lock = proc_table_lock.add()?;
        {
            let mut process = process_lock.write();
            process.context.set_page_table(unsafe { paging::ActivePageTable::new().address() });
            process.context.set_stack(stack.as_ptr() as usize + offset);

            Ok(process.clone())
        }
    }

    fn getid(&self) -> &ProcessId {
        &self.current_pid
    }

    fn ready(&mut self, id: ProcessId) {
        self.ready_list.write().push_back(id);
    }

    fn resched(&mut self) {
        let mut ready_list_lock = self.ready_list.write();

        if let Some(next_id) = ready_list_lock.pop_front() {
            let curr_id: ProcessId = self.getid().clone();

            let mut proc_table_lock = self.proc_table.write();

            let mut next = proc_table_lock.get(next_id).expect("Could not find new process").write();
            let mut current = proc_table_lock.get(curr_id).expect("No process currently running").write();

            next.set_state(State::Current);

            unsafe {
                current.context.switch_to(&mut next.context);
            }

            self.current_pid.set(next.pid.clone());
        }
    }

}

impl CoopScheduler {
    pub fn new() -> CoopScheduler {
        CoopScheduler {
            current_pid: ProcessId::new(0),
            proc_table: RwLock::new(ProcessList::new()),
            ready_list: RwLock::new(VecDeque::<ProcessId>::new()),
        }
    }
}
