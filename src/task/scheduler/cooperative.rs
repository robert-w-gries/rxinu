#![allow(dead_code)]
use alloc::collections::VecDeque;
use alloc::string::String;
use alloc::vec::Vec;
use core::ops::DerefMut;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::arch::context::Context;
use crate::sync::IrqSpinLock;
use crate::syscall::error::Error;
use crate::task::scheduler::{ProcessTable, Scheduling};
use crate::task::{Process, ProcessId, ProcessRef, State};

pub struct Cooperative {
    current_pid: AtomicUsize,
    inner: IrqSpinLock<CooperativeInner>,
    ticks: AtomicUsize,
}

struct CooperativeInner {
    proc_table: ProcessTable,
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
        let mut inner = self.inner.lock();

        let pid = inner.proc_table.next_pid()?;
        let proc: Process = Process::new(pid, name, 0, proc_entry);
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
        let state = {
            let proc = self.get_process(pid)?;
            let state = proc.read().state;
            state
        };

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
        self.modify_process(pid, |proc_ref| {
            proc_ref.write().set_state(State::Ready);
        })?;

        self.inner.lock().ready_list.push_back(pid);
        Ok(())
    }

    /// Safety: This method will deadlock if any scheduling locks are still held
    unsafe fn resched(&self) -> Result<(), Error> {
        let curr_id: ProcessId = self.get_pid();
        let next_id = if let Some(next_id) = self.inner.lock().ready_list.pop_front() {
            assert!(curr_id != next_id);
            next_id
        } else {
            return Ok(());
        };

        let curr_ref = self.modify_process(curr_id, |proc_ref| {
            let mut proc = proc_ref.write();
            if proc.state == State::Current {
                proc.set_state(State::Ready);
            }
        })?;

        let curr_proc: *mut Process = curr_ref.write().deref_mut() as *mut Process;

        if (*curr_proc).state == State::Ready {
            self.inner.lock().ready_list.push_back(curr_id);
        }

        let next_ref = self.modify_process(next_id, |proc_ref| {
            let mut proc = proc_ref.write();

            assert!(proc.kstack.is_some());
            proc.set_state(State::Current);
        })?;

        let next_proc = next_ref.write().deref_mut() as *mut Process;

        self.current_pid.store(next_id.0, Ordering::SeqCst);

        (*curr_proc).switch_to(&*next_proc);

        Ok(())
    }

    fn tick(&self) {
        // ticks don't matter for cooperative scheduling
    }

    fn unready(&self, pid: ProcessId) -> Result<(), Error> {
        let mut inner = self.inner.lock();

        if let Some(index) = inner.ready_list.iter().position(|x| *x == pid) {
            inner.ready_list.remove(index);
            Ok(())
        } else {
            Err(Error::BadPid)
        }
    }
}

impl Cooperative {
    pub fn new() -> Cooperative {
        Cooperative {
            current_pid: AtomicUsize::new(ProcessId::NULL_PROCESS.get_usize()),
            inner: IrqSpinLock::new(CooperativeInner {
                proc_table: ProcessTable::new(),
                ready_list: VecDeque::<ProcessId>::new(),
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
}
