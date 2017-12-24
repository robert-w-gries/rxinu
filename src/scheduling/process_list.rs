use alloc::Vec;
use alloc::arc::Arc;
use alloc::btree_map::{self, BTreeMap};
use core::fmt;
use core::result::Result;
use scheduling::{Process, ProcessId, State};
use spin::RwLock;
use syscall::error::Error;

pub struct ProcessList {
    map: BTreeMap<ProcessId, Arc<RwLock<Process>>>,
    next_id: usize,
}

impl fmt::Debug for ProcessList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ProcessList {{ map: {:#?} }}", self.map)
    }
}

impl ProcessList {
    pub fn new() -> Self {
        let mut new_list: BTreeMap<ProcessId, Arc<RwLock<Process>>> = BTreeMap::new();

        let mut null_process: Process = Process::new(ProcessId::NULL_PROCESS);
        null_process.state = State::Current;
        null_process.kstack = Some(Vec::new());

        new_list.insert(ProcessId::NULL_PROCESS, Arc::new(RwLock::new(null_process)));

        ProcessList {
            map: new_list,
            next_id: 1,
        }
    }

    pub fn get(&self, id: ProcessId) -> Option<&Arc<RwLock<Process>>> {
        self.map.get(&id)
    }

    pub fn iter(&self) -> btree_map::Iter<ProcessId, Arc<RwLock<Process>>> {
        self.map.iter()
    }

    pub fn add(&mut self) -> Result<&Arc<RwLock<Process>>, Error> {
        // We need to reset our search for an empty table if starting at the end
        if self.next_id >= super::MAX_PROCS {
            self.next_id = 1;
        }

        while self.map.contains_key(&ProcessId(self.next_id)) {
            self.next_id += 1;
        }

        if self.next_id >= super::MAX_PROCS {
            Err(Error::TryAgain)
        } else {
            let id: ProcessId = ProcessId(self.next_id);
            self.next_id += 1;

            assert!(
                self.map
                    .insert(id, Arc::new(RwLock::new(Process::new(id))))
                    .is_none(),
                "Process id already exists!"
            );

            Ok(self.map.get(&id).expect("Failed to add new process"))
        }
    }

    pub fn remove(&mut self, id: ProcessId) -> Option<Arc<RwLock<Process>>> {
        self.map.remove(&id)
    }
}
