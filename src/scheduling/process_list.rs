use alloc::btree_map::{self, BTreeMap};
use core::result::Result;
use scheduling::{Process, ProcessId};
use syscall::error::Error;

pub struct ProcessList<T> {
    collection: T,
    next_id: usize,
}

impl ProcessList<BTreeMap<ProcessId, Process>> {
    pub fn new() -> Self {
        ProcessList {
            collection: BTreeMap::new(),
            next_id: 1,
        }
    }

    pub fn get(&self, id: ProcessId) -> Option<&Process> {
        self.collection.get(&id)
    }

    pub fn iter(&self) -> btree_map::Iter<ProcessId, Process> {
        self.collection.iter()
    }

    pub fn add(&mut self) -> Result<&Process, Error> {
        // We need to reset our search for an empty table if starting at the end
        if self.next_id >= super::MAX_PROCS {
            self.next_id = 1;
        }

        while self.collection.contains_key(&ProcessId::new(self.next_id)) {
            self.next_id += 1;
        }

        if self.next_id >= super::MAX_PROCS {
            Err(Error::TryAgain)
        } else {
            let id: ProcessId = ProcessId::new(self.next_id);
            self.next_id += 1;

            assert!(self.collection.insert(id.clone(), Process::new(id.clone())).is_none(),
                    "Process id already exists!"); 

            Ok(self.collection.get(&id).expect("Failed to add new process"))
        }
    }

    pub fn remove(&mut self, id: ProcessId) -> Option<Process> {
        self.collection.remove(&id)
    }
}