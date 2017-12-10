use alloc::btree_map::{self, BTreeMap};
use core::result::Result;
use scheduling::{Process, ProcessId, State};
use spin::RwLock;
use syscall::error::Error;

pub struct ProcessList {
    collection: BTreeMap<ProcessId, RwLock<Process>>,
    next_id: usize,
}

impl ProcessList {
    pub fn new() -> Self {
        let mut new_list: BTreeMap<ProcessId, RwLock<Process>> = BTreeMap::new();

        let mut null_process: Process = Process::new(ProcessId::NULL_PROCESS);
        null_process.state = State::Current;

        new_list.insert(ProcessId::NULL_PROCESS, RwLock::new(null_process));

        ProcessList {
            collection: new_list,
            next_id: 1,
        }
    }

    pub fn get(&self, id: ProcessId) -> Option<&RwLock<Process>> {
        self.collection.get(&id)
    }

    pub fn iter(&self) -> btree_map::Iter<ProcessId, RwLock<Process>> {
        self.collection.iter()
    }

    pub fn add(&mut self) -> Result<&RwLock<Process>, Error> {
        // We need to reset our search for an empty table if starting at the end
        if self.next_id >= super::MAX_PROCS {
            self.next_id = 1;
        }

        while self.collection.contains_key(&ProcessId(self.next_id)) {
            self.next_id += 1;
        }

        if self.next_id >= super::MAX_PROCS {
            Err(Error::TryAgain)
        } else {
            let id: ProcessId = ProcessId(self.next_id);
            self.next_id += 1;

            assert!(self.collection.insert(id, RwLock::new(Process::new(id))).is_none(), "Process id already exists!");

            Ok(self.collection.get(&id).expect("Failed to add new process"))
        }
    }

    pub fn remove(&mut self, id: ProcessId) -> Option<RwLock<Process>> {
        self.collection.remove(&id)
    }
}
