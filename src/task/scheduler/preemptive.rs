use alloc::collections::BinaryHeap;
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::DerefMut;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::arch::context::Context;
use crate::arch::interrupts;
use crate::sync::IrqSpinLock;
use crate::syscall::error::Error;
use crate::task::scheduler::{ProcessTable, Scheduling};
use crate::task::{Process, ProcessId, ProcessRef, State};

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

        let pid = inner.proc_table.next_pid()?;
        let proc: Process = Process::new(pid, name, prio, proc_entry);
        inner.proc_table.add(proc)?;

        Ok(pid)
    }

    /// Get current process id
    fn get_pid(&self) -> ProcessId {
        ProcessId(self.current_pid.load(Ordering::SeqCst))
    }

    /// Get a reference to a process given a ProcessId
    fn get_process(&self, pid: ProcessId) -> Result<ProcessRef, Error> {
        match self.inner.lock().proc_table.get(pid) {
            Some(proc_ref) => Ok(proc_ref.clone()),
            None => Err(Error::BadPid),
        }
    }

    /// Scheduler's method to kill processes
    /// Currently, we just mark the process as FREE and leave its memory in the proc table
    fn kill(&self, pid: ProcessId) -> Result<(), Error> {
        interrupts::disable_then_execute(|| {
            let state = self.get_process(pid)?.state();

            self.modify_process(pid, |proc_ref| {
                let mut proc = proc_ref.write();

                // Free memory allocated to process
                proc.set_state(State::Free);
                proc.kstack = None;
            })?;

            match state {
                State::Current => unsafe {
                    self.resched()?;
                },
                State::Free => (),
                State::Ready => {
                    self.unready(pid)?;
                    self.inner.lock().proc_table.remove(pid);
                }
                State::Suspended => {
                    self.inner.lock().proc_table.remove(pid);
                }
                // TODO: Handle killing of waiting process
                State::Wait => panic!("Killing waiting processes is currently not supported"),
            }

            Ok(())
        })
    }

    /// Modify a process, given a ProcessId, and return a reference to it
    fn modify_process<F>(&self, pid: ProcessId, modify_fn: F) -> Result<ProcessRef, Error>
    where
        F: Fn(&ProcessRef),
    {
        if let Some(proc_ref) = self.inner.lock().proc_table.get(pid) {
            modify_fn(proc_ref);
            Ok(proc_ref.clone())
        } else {
            Err(Error::BadPid)
        }
    }

    /// Add process to ready list
    fn ready(&self, pid: ProcessId) -> Result<(), Error> {
        let proc_ref = self.modify_process(pid, |proc_ref| {
            proc_ref.write().set_state(State::Ready);
        })?;

        self.inner.lock().ready_list.push(proc_ref);

        Ok(())
    }

    /// Safety: This method will deadlock if any scheduling locks are still held
    unsafe fn resched(&self) -> Result<(), Error> {
        interrupts::disable_then_execute(|| {
            let curr_id: ProcessId = self.get_pid();

            let next_proc: *const Process =
                if let Some(next_ref) = self.inner.lock().ready_list.pop() {
                    let mut next_lock = next_ref.write();

                    assert!(next_lock.kstack.is_some());
                    assert!(curr_id != next_lock.pid);

                    next_lock.set_state(State::Current);

                    next_lock.deref_mut() as *const Process
                } else {
                    return Ok(());
                };

            // Process Aging
            // Prevent process starvation by increasing all ready process priorities
            self.age_processes();

            let current_ref = self.modify_process(curr_id, |proc_ref| {
                let mut proc = proc_ref.write();

                if proc.state == State::Current {
                    proc.set_state(State::Ready);
                }
            })?;

            match current_ref.read().state {
                State::Ready => {
                    self.inner.lock().ready_list.push(current_ref.clone());
                }
                State::Free => {
                    self.inner.lock().proc_table.remove(curr_id);
                }
                _ => (),
            }

            let current_proc: *mut Process = current_ref.write().deref_mut() as *mut Process;

            self.current_pid.store((*next_proc).pid.0, Ordering::SeqCst);

            (*current_proc).switch_to(&*next_proc);

            Ok(())
        })
    }

    fn tick(&self) {
        //This counter variable is updated every time an timer interrupt occurs. The timer is set to
        //interrupt every 2ms, so this means a reschedule will occur if 20ms have passed.
        if self.ticks.fetch_add(1, Ordering::SeqCst) >= 10 {
            self.ticks.store(0, Ordering::SeqCst);

            unsafe {
                self.resched().unwrap();
            }
        }
    }

    fn unready(&self, pid: ProcessId) -> Result<(), Error> {
        let mut inner = self.inner.lock();

        inner.ready_list = inner
            .ready_list
            .clone()
            .into_iter()
            .filter(|proc_ref| proc_ref.read().pid != pid)
            .collect();

        Ok(())
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
            ticks: AtomicUsize::new(0),
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
        };

        self.inner
            .lock()
            .proc_table
            .insert(ProcessId::NULL_PROCESS, ProcessRef::new(null_process));
    }

    fn age_processes(&self) {
        for p in self.inner.lock().ready_list.iter() {
            p.write().priority += 1;
        }
    }
}
