use alloc::{BTreeMap, VecDeque};
use alloc::boxed::Box;
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

        let mut stack: Box<[usize]> = vec![0; INIT_STK_SIZE].into_boxed_slice();

        // TODO: Investigate proper offset
        // let offset = stack.len() - mem::size_of::<usize>();
        let offset = stack.len() - 1;
        let offset2 = (stack.len() * mem::size_of::<usize>()) - mem::size_of::<usize>();

        unsafe {
            let func_ptr = stack.as_mut_ptr().offset(offset as isize);
            *(func_ptr as *mut usize) = func as usize;
        }

        let mut proc_table_lock = self.proc_table.write();

        let process_lock = proc_table_lock.add()?;
        {
            let mut process = process_lock.write();
            process.context.set_page_table(unsafe { paging::ActivePageTable::new().address() });
            process.context.set_stack((stack.as_ptr() as usize) + offset2);

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
            let mut proc_table_lock = self.proc_table.write();

            let mut next = proc_table_lock.get(next_id).expect("Could not find new process").write();
            next.set_state(State::Current);

            let curr_id: ProcessId = self.getid().clone();
            let mut current = proc_table_lock.get(curr_id).expect("Could not find current process").write();

            unsafe {
                current.context.switch_to(&mut next.context);
            }

            self.current_pid = next.pid.clone();
        }
    }
}

impl CoopScheduler {
    pub fn new() -> CoopScheduler {
        CoopScheduler {
            current_pid: ProcessId::NULL,
            proc_table: RwLock::new(ProcessList::new()),
            ready_list: RwLock::new(VecDeque::<ProcessId>::new()),
        }
    }
}
