use alloc::String;
use alloc::boxed::Box;
use arch::context::Context;
use scheduling::{ProcessId, Scheduler};

#[derive(Clone, Eq, PartialEq)]
pub enum State {
    Free,
    Current,
    Suspended,
    Ready,
}

#[derive(Clone)]
pub struct Priority(u64);

#[derive(Clone)]
pub struct Process {
    pub pid: ProcessId,
    pub state: State,
    pub prio: Priority,
    pub context: Context,
    pub stack: Option<Box<[u8]>>,
    pub name: String,
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

/// When a process returns, it pops off a return instruction pointer then jumps to it
/// This is the function pointer stored on the stack when a process is created
/// Note: We are inside the SCHEDULER singleton lock and cannot re-aquire the reference
/// So we use dynamic dispatch to get scheduler Trait Object then call kill method
pub unsafe fn process_ret() {
    use scheduling::{DoesScheduling, scheduler};
kprintln!("Return from process!");

    //let scheduler: &mut Scheduler;
    //asm!("pop $0" : "=r"(scheduler) : : "memory" : "intel", "volatile");

//kprintln!("Scheduler address = 0x{:x}", scheduler as *mut _ as usize);
    let curr_id: ProcessId = scheduler().getid().clone();
kprintln!("Cloned!");
    scheduler().kill(curr_id);
kprintln!("Done killing!");
}
