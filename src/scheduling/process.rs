use alloc::String;
use alloc::Vec;
use arch::context::Context;
use core::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum State {
    Free,
    Current,
    Suspended,
    Ready,
}

#[derive(Clone)]
pub struct Priority(u64);

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
    pub fn new(id: ProcessId) -> Process {
        Process {
            pid: id,
            state: State::Suspended,
            prio: Priority(0),
            context: Context::new(),
            kstack: None,
            name: String::from("NULL"),
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
/// Note:
/// To support multiple scheduler objects, use dynamic dispatch
/// The parent scheduler object will always be on the stack
#[naked]
pub unsafe extern "C" fn process_ret() {
    use alloc::boxed::Box;
    use scheduling::{DoesScheduling, SCHEDULER};

    let scheduler_ptr: *mut &DoesScheduling;
    asm!("pop $0" : "=r"(scheduler_ptr) : : "memory" : "intel", "volatile");

    let scheduler = Box::from_raw(scheduler_ptr);

    //let curr_id: ProcessId = scheduler.getid();
    //scheduler.kill(curr_id);
    let curr_id: ProcessId = SCHEDULER.getid();
    SCHEDULER.kill(curr_id);
}
