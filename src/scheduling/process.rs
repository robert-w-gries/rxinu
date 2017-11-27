use alloc::String;
use alloc::boxed::Box;
use alloc::btree_map::{self, BTreeMap};
use arch::context::Context;
use core::ops::Add;
use core::result::Result;
use core::sync::atomic::AtomicUsize;
use syscall::error::Error;

pub enum State {
    Free,
    Current,
    Suspended,
    Ready,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct ProcessId(pub AtomicUsize);

impl Add for ProcessId {
    type Output = ProcessId;

    fn add(self, other: ProcessId) -> ProcessId {
        ProcessId(self.0 + other.0)
    }
}

struct Priority(u64);

pub struct Process {
    pid: ProcessId,
    pub state: State,
    prio: Priority,
    context: Context,
    stack: Option<Box<[u8]>>,
    name: String,
}

impl Process {
    pub fn new(id: ProcessId) -> Process {
        Process {
            pid: ProcessId(0 as u64),
            state: State::Suspended,
            prio: Priority(0),
            context: Context::new(),
            stack: None,
            name: String::new("NEW"),
        }
    }

    pub fn context(&mut self) -> Context {
        self.context
    }

    pub fn pid(&mut self) -> ProcessId {
        self.pid
    }

    pub fn set_page_table(&mut self, address: usize) {
        self.context().set_page_table(address);
    }

    pub fn set_stack(&mut self, address: usize) {
        self.context().set_stack(address);
    }
}

pub struct ProcessList<T> {
    collection: T,
    next_id: ProcessId,
}

impl ProcessList<BTreeMap<ProcessId, Process>> {
    pub fn new() -> Self {
        ProcessList {
            collection: BTreeMap::new(),
            next_id: 1,
        }
    }

    pub fn get(&self, id: ProcessId) -> Option<Process> {
        self.collection.get(&id)
    }

    pub fn iter(&self) -> btree_map::Iter<ProcessId, Process> {
        self.collection.iter()
    }

    pub fn add(&mut self) -> Result<Process, Error> {
        // We need to reset our search for an empty table if starting at the end
        if self.next_id >= super::MAX_PROCS {
            self.next_id = ProcessId(1);
        }

        while self.collection.contains_key(&ProcessId::from(self.next_id)) {
            self.next_id += 1;
        }

        if self.next_id >= super::MAX_PROCS {
            Err(Error::new(Error::TryAgain))
        } else {
            let id: ProcessId = ProcessId(self.next_id.clone());
            self.next_id += 1;

            assert!(self.collection.insert(id, Process::new(id).is_none())); 

            Ok(self.collection.get(&id).expect("Failed to add new process"))
        }
    }

    pub fn remove(&mut self, id: ProcessId) -> Option<Process> {
        self.collection.remove(&id)
    }
}
