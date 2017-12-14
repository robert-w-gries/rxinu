use alloc::{String, Vec};
use arch::context::Context;
use scheduling::Scheduler;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum State {
    Free,
    Current,
    Suspended,
    Ready,
}

#[derive(Clone, Debug)]
pub struct Priority(u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ProcessId(pub usize);

impl ProcessId {
    pub const NULL_PROCESS: ProcessId = ProcessId(0);

    pub fn get_usize(&self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug)]
pub struct Process {
    pub pid: ProcessId,
    pub name: String,
    pub state: State,
    pub prio: Priority,
    pub context: Context,
    pub kstack: Option<Vec<usize>>,
}

impl Process {
    pub fn new(id: ProcessId) -> Process {
        Process {
            pid: id,
            state: State::Suspended,
            prio: Priority(0),
            context: Context::new(),
            kstack: None,
            name: String::from("NEW"),
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
    use scheduling::DoesScheduling;

    let scheduler: &mut Scheduler;
    asm!("pop $0" : "=r"(scheduler) : : "memory" : "intel", "volatile");

    let curr_id: ProcessId = scheduler.getid();
    scheduler.kill(curr_id);
}
