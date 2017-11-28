use alloc::{BTreeMap, VecDeque};
use core::cmp::Ord;
use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering};
use scheduling::{Process, ProcessId, ProcessList, State, Scheduler, INIT_STK_SIZE};
use spin::Mutex;
use syscall::error::Error;

lazy_static! {
   pub static ref SCHEDULER: CoopScheduler = CoopScheduler::new();
}

pub struct CoopScheduler {
    current_pid: ProcessId,
    proc_table: ProcessList<BTreeMap<ProcessId, Process>>,
    ready_list: VecDeque<ProcessId>,
}

impl Scheduler for CoopScheduler {
    fn create(&mut self, func: extern fn()) -> Result<&Process, Error> {
        use arch::context::Context;
        use arch::memory::paging;

        let mut stack = vec![0; INIT_STK_SIZE].into_boxed_slice();

        let offset = stack.len() - mem::size_of::<usize>();
        unsafe {
            let func_ptr = stack.as_mut_ptr().offset(offset as isize);
            *(func_ptr as *mut usize) = func as usize;
        }

        let mut process: &Process = self.proc_table.add()?;
        process.context_mut().set_page_table(unsafe { paging::ActivePageTable::new().address() });
        process.context_mut().set_stack(stack.as_ptr() as usize + offset);

        Ok(process)
    }

    fn getid(&self) -> &ProcessId {
        &self.current_pid
    }

    fn ready(&mut self, id: ProcessId) {
        self.ready_list.push_back(id);
    }

    fn resched(&self) {
        if let Some(next_id) = self.ready_list.pop_front() {
            let mut next: &Process = self.proc_table
                                          .get(next_id)
                                          .expect("Could not find new process");

            next.set_state(State::Current);

            let mut current: Process = *self.proc_table
                                            .get(*self.getid())
                                            .expect("No process currently running");

            unsafe {
                current.context_mut().switch_to(next.context_mut());
            }

            self.current_pid.set(next.pid());
        }
    }
}

impl CoopScheduler {
    fn new() -> CoopScheduler {
        CoopScheduler {
            current_pid: ProcessId::new(0),
            proc_table: ProcessList::new(),
            ready_list: VecDeque::<ProcessId>::new(),
        }
    }
}
