use alloc::arc::Arc;
use alloc::{BinaryHeap, String, Vec};
use arch::context::Context;
use core::cmp;
use core::ops::DerefMut;
use core::sync::atomic::{self, AtomicUsize, ATOMIC_USIZE_INIT};
use spin::RwLock;
use sync::IrqSpinLock;
use syscall::error::Error;
use task::scheduler::Scheduling;
use task::{Process, ProcessId, ProcessTable, State};

pub struct Preemptive {
    current_pid: AtomicUsize,
    inner: IrqSpinLock<PreemptiveInner>,
    ticks: AtomicUsize,
}

struct PreemptiveInner {
    proc_table: ProcessTable,
    ready_list: BinaryHeap<ProcessRef>,
}

impl Scheduling for Preemptive {
    /// Add process to process table
    fn create(
        &self,
        name: String,
        prio: usize,
        proc_entry: extern "C" fn(),
    ) -> Result<ProcessId, Error> {
        let mut inner = self.inner.lock();

        let pid = inner.proc_table.get_next_pid()?;
        let proc: Process = Process::new(pid, name, prio, proc_entry);
        inner.proc_table.add(proc)?;

        Ok(pid)
    }

    /// Get current process id
    fn getid(&self) -> ProcessId {
        ProcessId(self.current_pid.load(atomic::Ordering::SeqCst))
    }

    /// Scheduler's method to kill processes
    /// Currently, we just mark the process as FREE and leave its memory in the proc table
    fn kill(&self, id: ProcessId) {
        // We need to scope the manipulation of the process so we don't deadlock in resched()
        {
            let inner = self.inner.lock();

            let proc_lock = inner
                .proc_table
                .get(id)
                .expect("Could not find process to kill");

            let mut proc = proc_lock.write();

            proc.set_state(State::Free);
            proc.kstack = None;
            drop(&mut proc.name);
        }

        unsafe {
            self.resched();
        }
    }

    /// Add process to ready list
    fn ready(&self, id: ProcessId) -> Result<(), Error> {
        let mut inner = self.inner.lock();

        let proc_ref = {
            if let Some(proc_ref) = inner.proc_table.get(id) {
                let mut proc = proc_ref.write();
                proc.set_state(State::Ready);
                Arc::clone(proc_ref)
            } else {
                return Err(Error::ProcessNotFound);
            }
        };

        inner.ready_list.push(ProcessRef(proc_ref));

        Ok(())
    }

    /// Safety: This method will deadlock if any scheduling locks are still held
    unsafe fn resched(&self) {
        // Important: Ensure lock is dropped before context switch
        let mut inner = self.inner.lock();

        let curr_id: ProcessId = self.getid();

        let next_proc: *const Process = if let Some(next_ref) = inner.ready_list.pop() {
            let mut next_lock = next_ref.0.write();

            assert!(next_lock.kstack.is_some());
            assert!(curr_id != next_lock.pid);

            next_lock.set_state(State::Current);

            next_lock.deref_mut() as *const Process
        } else {
            return;
        };

        {
            for (_, p) in inner
                .proc_table
                .map
                .iter()
                .filter(|&(_, proc)| proc.read().state == State::Ready)
            {
                p.write().priority += 1;
            }
        }

        {
            let curr_ref = {
                let proc_ref = inner
                    .proc_table
                    .get(curr_id)
                    .expect("resched() - Could not find current process in process table");
                Arc::clone(proc_ref)
            };
            let curr = curr_ref.read();
            if curr.state == State::Current {
                inner.ready_list.push(ProcessRef(Arc::clone(&curr_ref)));
            }
        }

        let current_proc: *mut Process = {
            let curr_ref = {
                let proc_ref = inner
                    .proc_table
                    .get(curr_id)
                    .expect("resched() - Could not find current process in process table");
                Arc::clone(proc_ref)
            };
            let mut curr = curr_ref.write();

            match curr.state {
                State::Current => {
                    curr.set_state(State::Ready);
                }
                State::Free => {
                    inner.proc_table.remove(curr_id);
                }
                _ => (),
            };

            curr.deref_mut() as *mut Process
        };

        self.current_pid
            .store((*next_proc).pid.0, atomic::Ordering::SeqCst);

        // Drop locks to prevent deadlock after context switch
        inner.release();

        (*current_proc).switch_to(&*next_proc);
    }

    fn tick(&self) {
        //This counter variable is updated every time an timer interrupt occurs. The timer is set to
        //interrupt every 2ms, so this means a reschedule will occur if 20ms have passed.
        if self.ticks.fetch_add(1, atomic::Ordering::SeqCst) >= 10 {
            self.ticks.store(0, atomic::Ordering::SeqCst);

            unsafe {
                self.resched();
            }
        }
    }
}

impl Preemptive {
    pub fn new() -> Preemptive {
        Preemptive {
            current_pid: AtomicUsize::new(ProcessId::NULL_PROCESS.get_usize()),
            inner: IrqSpinLock::new(PreemptiveInner {
                proc_table: ProcessTable::new(),
                ready_list: BinaryHeap::<ProcessRef>::new(),
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
            .insert(ProcessId::NULL_PROCESS, Arc::new(RwLock::new(null_process)));
    }
}

struct ProcessRef(Arc<RwLock<Process>>);

impl Ord for ProcessRef {
    fn cmp(&self, other: &ProcessRef) -> cmp::Ordering {
        self.0.read().priority.cmp(&other.0.read().priority)
    }
}

impl PartialOrd for ProcessRef {
    fn partial_cmp(&self, other: &ProcessRef) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for ProcessRef {}

impl PartialEq for ProcessRef {
    fn eq(&self, other: &ProcessRef) -> bool {
        self.0.read().priority == other.0.read().priority
    }
}
