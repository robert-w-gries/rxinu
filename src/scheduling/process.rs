use alloc::String;
use alloc::boxed::Box;
use arch::context::Context;
use scheduling::ProcessId;

#[derive(Clone)]
pub enum State {
    Free,
    Current,
    Suspended,
    Ready,
}

#[derive(Clone)]
struct Priority(u64);

#[derive(Clone)]
pub struct Process {
    pub pid: ProcessId,
    state: State,
    prio: Priority,
    pub context: Context,
    stack: Option<Box<[u8]>>,
    name: String,
}

impl Process {
    pub fn new(id: ProcessId) -> Process {
        Process {
            pid: id,
            state: State::Suspended,
            prio: Priority(0),
            context: Context::new(),
            stack: None,
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
