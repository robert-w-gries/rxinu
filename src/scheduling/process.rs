use alloc::String;
use alloc::boxed::Box;
use arch::context::Context;
use scheduling::ProcessId;

pub enum State {
    Free,
    Current,
    Suspended,
    Ready,
}

struct Priority(u64);

pub struct Process {
    pid: ProcessId,
    state: State,
    prio: Priority,
    context: Context,
    stack: Option<Box<[u8]>>,
    name: String,
}

impl Process {
    pub fn new(id: ProcessId) -> Process {
        Process {
            pid: ProcessId::new(0),
            state: State::Suspended,
            prio: Priority(0),
            context: Context::new(),
            stack: None,
            name: String::from("NEW"),
        }
    }

    pub fn context_mut(&self) -> &mut Context {
        &mut self.context
    }

    pub fn pid(&mut self) -> ProcessId {
        self.pid
    }

    pub fn set_state(&mut self, new_state: State) {
        self.state = new_state;
    }

    pub fn set_page_table(&mut self, address: usize) {
        self.context_mut().set_page_table(address);
    }

    pub fn set_stack(&mut self, address: usize) {
        self.context_mut().set_stack(address);
    }
}
