use alloc::String;
use alloc::Vec;
use arch::context::Context;
use core::fmt;
use task::INIT_STK_SIZE;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum State {
    Free,
    Current,
    Suspended,
    Ready,
}

#[derive(Clone)]
pub struct Priority(pub u64);

impl fmt::Debug for Priority {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Priority({})", self.0)
    }
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

#[derive(Clone)]
pub struct Process {
    pub pid: ProcessId,
    pub name: String,
    pub state: State,
    pub prio: Priority,
    pub context: Context,
    pub kstack: Option<Vec<usize>>,
}

impl fmt::Debug for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = f.debug_struct("Process");
        s.field("pid", &self.pid);
        s.field("name", &self.name);
        s.field("prio", &self.prio);
        s.field("context", &self.context);
        match self.kstack {
            Some(ref stk) => s.field("kstack", &(stk.as_ptr() as usize)),
            None => s.field("kstack", &self.kstack),
        };
        s.finish()
    }
}

impl Process {
    pub fn new(id: ProcessId, name: String, proc_entry: extern "C" fn()) -> Process {
        // Allocate stack
        let mut stack: Vec<usize> = vec![0; INIT_STK_SIZE];
        let stack_top = unsafe { stack.as_mut_ptr().add(INIT_STK_SIZE) };

        Process {
            pid: id,
            state: State::Suspended,
            prio: Priority(0),
            context: Context::new(stack_top as *mut u8, proc_entry as usize),
            kstack: Some(stack),
            name: name,
        }
    }

    pub fn set_state(&mut self, new_state: State) {
        self.state = new_state;
    }

    pub fn set_page_table(&mut self, address: usize) {
        self.context.set_page_table(address);
    }

    pub fn set_stack(&mut self, address: usize) {
        self.context.set_stack(address);
    }
}

/// Once the process it completed, kill it
///
/// When a process returns, it pops an instruction pointer off the stack then jumps to it
/// The instruction pointer on the stack points to this function
pub unsafe extern "C" fn process_ret() {
    use task::{Scheduling, SCHEDULER};

    let curr_id: ProcessId = SCHEDULER.getid();
    SCHEDULER.kill(curr_id);
}
