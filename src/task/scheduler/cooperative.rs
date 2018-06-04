#![allow(dead_code)]
use alloc::btree_map::BTreeMap;
use alloc::{String, Vec, VecDeque};
use arch::context::Context;
use core::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use spin::Mutex;
use syscall::error::Error;
use task::scheduler::Scheduling;
use task::{Process, ProcessId, State, MAX_PID};

pub struct Cooperative {
    current_pid: AtomicUsize,
    inner: Mutex<CooperativeInner>,
    ticks: AtomicUsize,
}

struct CooperativeInner {
    next_pid: usize,
    proc_table: BTreeMap<ProcessId, Process>,
    ready_list: VecDeque<ProcessId>,
}

impl Scheduling for Cooperative {
    /// Add process to process table
    fn create(
        &self,
        name: String,
        _prio: usize,
        proc_entry: extern "C" fn(),
    ) -> Result<ProcessId, Error> {
        let pid = self.get_next_pid()?;
        let proc: Process = Process::new(pid, name, 0, proc_entry);

        match self.inner.lock().proc_table.insert(pid, proc) {
            Some(_) => Err(Error::BadPid),
            None => Ok(pid),
        }
    }

    /// Get current process id
    fn getid(&self) -> ProcessId {
        ProcessId(self.current_pid.load(Ordering::SeqCst))
    }

    /// Scheduler's method to kill processes
    /// Currently, we just mark the process as FREE and leave its memory in the proc table
    fn kill(&self, id: ProcessId) -> Result<(), Error> {
        let mut inner = self.inner.lock();

        if let Some(proc_lock) = inner.proc_table.get_mut(&id) {
            proc_lock.set_state(State::Free);
            proc_lock.kstack = None;
            drop(&mut proc_lock.name);
        } else {
            return Err(Error::BadPid);
        };

        drop(inner);

        unsafe {
            self.resched();
        }

        Ok(())
    }

    /// Add process to ready list
    fn ready(&self, id: ProcessId) -> Result<(), Error> {
        self.inner.lock().ready_list.push_back(id);
        Ok(())
    }

    /// Safety: This method will deadlock if any scheduling locks are still held
    unsafe fn resched(&self) {
        // Important: Ensure lock is dropped before context switch
        let mut inner = self.inner.lock();

        let curr_id: ProcessId = self.getid();
        let next_id = if let Some(next_id) = inner.ready_list.pop_front() {
            assert!(curr_id != next_id);
            next_id
        } else {
            return;
        };

        let current_proc_ptr = {
            let mut current_proc = inner
                .proc_table
                .get_mut(&curr_id)
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
                .get_mut(&next_id)
                .expect("Process ID in ready list is not in process table");

            assert!(next_proc.kstack.is_some());

            next_proc.set_state(State::Current);

            next_proc as *const Process
        };

        if (*current_proc_ptr).state == State::Ready {
            inner.ready_list.push_back(curr_id);
        }

        self.current_pid.store(next_id.0, Ordering::SeqCst);

        // Drop locks to prevent deadlock after context switch
        drop(inner);

        (*current_proc_ptr).switch_to(&*next_proc_ptr);
    }

    fn tick(&self) {
        use arch::interrupts;

        //This counter variable is updated every time an timer interrupt occurs. The timer is set to
        //interrupt every 2ms, so this means a reschedule will occur if 20ms have passed.
        if self.ticks.fetch_add(1, Ordering::SeqCst) >= 10 {
            self.ticks.store(0, Ordering::SeqCst);

            // Find another process to run while interrupts are disabled
            interrupts::mask_then_restore(|| unsafe {
                self.resched();
            });
        }
    }
}

impl Cooperative {
    pub fn new() -> Cooperative {
        Cooperative {
            current_pid: AtomicUsize::new(ProcessId::NULL_PROCESS.get_usize()),
            inner: Mutex::new(CooperativeInner {
                next_pid: 1,
                proc_table: BTreeMap::<ProcessId, Process>::new(),
                ready_list: VecDeque::<ProcessId>::new(),
            }),
            ticks: ATOMIC_USIZE_INIT,
        }
    }

    /// Safety: Interrupts must be disabled during this initialization
    pub unsafe fn init(&self) {
        let null_process = Process {
            pid: ProcessId::NULL_PROCESS,
            name: String::from("NULL"),
            state: State::Current,
            context: Context::empty(),
            kstack: Some(Vec::new()),
            priority: 0,
            intr_mask: (0, 0),
        };

        self.inner
            .lock()
            .proc_table
            .insert(ProcessId::NULL_PROCESS, null_process);
    }

    fn get_next_pid(&self) -> Result<ProcessId, Error> {
        let mut inner = self.inner.lock();

        while inner.proc_table.contains_key(&ProcessId(inner.next_pid)) && inner.next_pid < MAX_PID
        {
            inner.next_pid += 1;
        }

        match inner.next_pid {
            MAX_PID => {
                inner.next_pid = 1;
                Err(Error::TryAgain)
            }
            pid => {
                inner.next_pid += 1;
                Ok(ProcessId(pid))
            }
        }
    }
}
