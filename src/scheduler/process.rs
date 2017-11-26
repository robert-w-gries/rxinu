use arch::context::Context;

pub enum State {
    Free,
    Current,
    Suspended,
    Ready,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone, Copy)]
struct ProcessId(u64);

impl Add for ProcessId {
    type Output = ProcessId;

    fn add(self, other: ProcessId) -> ProcessId {
        ProcessId { self.0 + other.0 }
    }
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
    pub fn new(id: ProcessId) {
        pid: ProcessId,
        state: State::Suspended,
        prio: Priority(0),
        context: Context::new(),
        stack: None,
        name: "NEW",
    }

    pub fn set_page_table(&mut self, address: usize) {
        context.set_page_table(address);
    }

    pub fn set_stack(&mut self, address: usize) {
        context.set_stack(address);
    }
}

pub struct ProcessList<T> {
    collection: T,
    next_id: ProcessId,
}

impl ProcessList<BTreeMap> {
    pub fn new() -> Self {
        ProcessList {
            collection: BTreeMap::new(),
            next_id: 1,
        }
    }

    pub fn get(&self, id: ProcessId) -> Option<Process> {
        self.collection.get(&id)
    }

    pub fn current(&self) -> Option<Process> {
        self.collection.get(*CURRENT_PID.lock())
    }

    pub fn iter(&self) -> btree_map::Iter<ProcessId, Process> {
        self.collection.iter()
    }

    pub fn add(&mut self) -> Result<Process> {
        // We need to reset our search for an empty table if starting at the end
        if self.next_id >= super::MAX_PROCS {
            self.next_id = ProcessId(1);
        }

        while self.collection.contains_key(&ContextId::from(self.next_id)) {
            self.next_id += 1;
        }

        if self.next_id >= super::CONTEXT_MAX_CONTENTS {
            Err(Error::new(TryAgain))
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
