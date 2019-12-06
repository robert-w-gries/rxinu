use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::cmp;
use core::fmt;
use spin::{RwLock, RwLockReadGuard, RwLockWriteGuard};

use crate::arch::context::Context;
use crate::syscall::error::Error;
use crate::task::scheduler::{global_sched, Scheduling};

/// Once the process it completed, kill it
///
/// When a process returns, it pops an instruction pointer off the stack then jumps to it
/// The instruction pointer on the stack points to this function
pub unsafe extern "C" fn process_ret() {
    let curr_id: ProcessId = global_sched().get_pid();
    global_sched().kill(curr_id).unwrap();
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    Current,
    Free,
    Ready,
    Suspended,
    Wait,
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
// TODO: Remove the 'pub'
pub struct ProcessId(pub usize);

impl fmt::Debug for ProcessId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ProcessId({})", self.0)
    }
}

impl ProcessId {
    pub const NULL_PROCESS: ProcessId = ProcessId(0);

    pub fn get_usize(&self) -> usize {
        self.0
    }
}

pub const PROCESS_STACK_SIZE: usize = 1024 * 4;

#[derive(Clone)]
pub struct Process {
    pub context: Context,
    pub kstack: Option<Vec<usize>>,
    pub pid: Option<ProcessId>,
    pub priority: usize,
    pub name: String,
    pub state: State,
}

impl fmt::Debug for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = f.debug_struct("Process");
        s.field("context", &self.context);
        s.field("kstack", &match self.kstack {
            Some(ref stk) => Some(stk.as_ptr() as usize),
            None => None,
        });
        s.field("pid", &self.pid);
        s.field("priority", &self.priority);
        s.field("name", &self.name);
        s.field("state", &self.state);
        s.finish()
    }
}

impl Process {
    pub fn new(
        name: &str,
        priority: usize,
        proc_entry: extern "C" fn(),
    ) -> Box<Process> {
        let mut stack: Vec<usize> = vec![0; PROCESS_STACK_SIZE];
        let stack_top = unsafe { stack.as_mut_ptr().add(PROCESS_STACK_SIZE) };

        Box::new(Process {
            context: Context::new(stack_top as *mut u8, proc_entry as usize),
            kstack: Some(stack),
            pid: None,
            priority: priority,
            name: name.to_string(),
            state: State::Suspended,
        })
    }

    pub fn pid(&self) -> Option<ProcessId> {
        self.pid
    }

    pub fn set_state(&mut self, new_state: State) {
        self.state = new_state;
    }

    pub fn spawn(self) -> Result<(), Error> {
        // TODO don't unwrap!
        let pid = global_sched().add_process(self)?;
        global_sched().ready(pid)
    }

    pub unsafe fn switch_to(&mut self, next: &Process) {
        self.context.switch_to(&next.context);
    }
}

#[derive(Clone, Debug)]
pub struct ProcessRef(pub Arc<RwLock<Process>>);

impl ProcessRef {
    pub fn new(proc: Process) -> ProcessRef {
        ProcessRef(Arc::new(RwLock::new(proc)))
    }

    pub fn read<'a>(&'a self) -> RwLockReadGuard<'a, Process> {
        self.0.read()
    }

    pub fn write<'a>(&'a self) -> RwLockWriteGuard<'a, Process> {
        self.0.write()
    }

    pub fn set_state(&self, state: State) {
        self.write().state = state;
    }

    pub fn state(&self) -> State {
        self.read().state
    }
}

impl Ord for ProcessRef {
    fn cmp(&self, other: &ProcessRef) -> cmp::Ordering {
        self.read().priority.cmp(&other.read().priority)
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
        self.read().priority == other.0.read().priority
    }
}
