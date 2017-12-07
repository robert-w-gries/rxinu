use alloc::String;
use alloc::boxed::Box;
use arch::context::Context;
use scheduling::{ProcessId, Scheduler};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum State {
    Free,
    Current,
    Suspended,
    Ready,
}

#[derive(Clone, Debug)]
pub struct Priority(u64);

#[derive(Clone, Debug)]
pub struct Process {
    pub pid: ProcessId,
    pub name: String,
    pub state: State,
    pub prio: Priority,
    pub context: Context,
    pub stack: Option<Box<[u8]>>,
    pub scheduler: Option<*const Scheduler>,
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
            scheduler: None,
        }
    }

    pub fn set_state(&mut self, new_state: State) {
        self.state = new_state;
    }

    pub fn set_page_table(&mut self, address: usize) {
        self.context.set_page_table(address);
    }

    pub fn set_scheduler(&mut self, scheduler: &Scheduler) {
        self.scheduler = Some(scheduler as *const Scheduler);
    }

    pub fn set_stack(&mut self, address: usize) {
        self.context.set_stack(address);
    }
}

/// When a process returns, it pops off a return instruction pointer then jumps to it
/// This is the function pointer stored on the stack when a process is created
/// Note: We are inside the SCHEDULER singleton lock and cannot re-aquire the reference
/// So we use dynamic dispatch to get scheduler Trait Object then call kill method
#[naked]
pub unsafe fn process_ret() {
    use scheduling::{DoesScheduling, scheduler};

    let scheduler: &mut Scheduler;
    asm!("pop $0" : "=r"(scheduler) : : "memory" : "intel", "volatile");

    let curr_id: ProcessId = scheduler.getid().clone();
    scheduler.kill(curr_id);
}
