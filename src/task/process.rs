use alloc::arc::Arc;
use alloc::btree_map::BTreeMap;
use alloc::String;
use alloc::Vec;
use arch::context::Context;
use core::cmp;
use core::fmt;
use spin::RwLock;
use syscall::error::Error;

/// Once the process it completed, kill it
///
/// When a process returns, it pops an instruction pointer off the stack then jumps to it
/// The instruction pointer on the stack points to this function
pub unsafe extern "C" fn process_ret() {
    use task::scheduler::{global_sched, Scheduling};

    let curr_id: ProcessId = global_sched().get_pid();
    global_sched().kill(curr_id);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    Free,
    Current,
    Suspended,
    Ready,
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
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

const PROCESS_STACK_SIZE: usize = 1024 * 4;

#[derive(Clone)]
pub struct Process {
    pub pid: ProcessId,
    pub name: String,
    pub state: State,
    pub context: Context,
    pub kstack: Option<Vec<usize>>,
    pub priority: usize,
    pub intr_mask: (u8, u8),
}

impl fmt::Debug for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = f.debug_struct("Process");
        s.field("pid", &self.pid);
        s.field("name", &self.name);
        s.field("context", &self.context);
        match self.kstack {
            Some(ref stk) => s.field("kstack", &(stk.as_ptr() as usize)),
            None => s.field("kstack", &self.kstack),
        };
        s.field("priority", &self.priority);
        s.finish()
    }
}

impl Process {
    pub fn new(
        id: ProcessId,
        name: String,
        priority: usize,
        proc_entry: extern "C" fn(),
    ) -> Process {
        // Allocate stack
        let mut stack: Vec<usize> = vec![0; PROCESS_STACK_SIZE];
        let stack_top = unsafe { stack.as_mut_ptr().add(PROCESS_STACK_SIZE) };

        Process {
            pid: id,
            state: State::Suspended,
            context: Context::new(stack_top as *mut u8, proc_entry as usize),
            kstack: Some(stack),
            name: name,
            priority: priority,
            intr_mask: (0, 0),
        }
    }

    pub fn set_state(&mut self, new_state: State) {
        self.state = new_state;
    }

    pub unsafe fn switch_to(&mut self, next: &Process) {
        self.context.switch_to(&next.context);
    }
}

pub struct ProcessTable {
    pub map: BTreeMap<ProcessId, ProcessRef>,
    next_pid: usize,
}

impl ProcessTable {
    pub fn new() -> ProcessTable {
        ProcessTable {
            map: BTreeMap::new(),
            next_pid: 1,
        }
    }

    pub fn add(&mut self, proc: Process) -> Result<ProcessId, Error> {
        let pid = proc.pid;
        match self.map.insert(pid, ProcessRef(Arc::new(RwLock::new(proc)))) {
            // PID already used
            Some(_) => Err(Error::BadPid),
            None => Ok(pid),
        }
    }

    pub fn get(&self, pid: ProcessId) -> Option<&ProcessRef> {
        self.map.get(&pid)
    }

    pub fn get_next_pid(&mut self) -> Result<ProcessId, Error> {
        use task::MAX_PID;

        while self.map.contains_key(&ProcessId(self.next_pid)) && self.next_pid < MAX_PID {
            self.next_pid += 1;
        }

        match self.next_pid {
            MAX_PID => {
                self.next_pid = 1;
                Err(Error::TryAgain)
            }
            pid => {
                self.next_pid += 1;
                Ok(ProcessId(pid))
            }
        }
    }

    pub fn insert(
        &mut self,
        pid: ProcessId,
        proc: ProcessRef,
    ) -> Option<ProcessRef> {
        self.map.insert(pid, proc)
    }

    pub fn remove(&mut self, pid: ProcessId) -> Option<ProcessRef> {
        self.map.remove(&pid)
    }
}

#[derive(Clone)]
pub struct ProcessRef(pub Arc<RwLock<Process>>);

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
