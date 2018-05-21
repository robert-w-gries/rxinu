use alloc::btree_map::{self, BTreeMap};
use alloc::Vec;
use core::fmt;
use core::result::Result;
use task::{Process, ProcessId, State};
use syscall::error::Error;

pub struct ProcessList {
    map: BTreeMap<ProcessId, Process>,
    next_id: usize,
}

impl fmt::Debug for ProcessList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ProcessList {{ map: {:#?} }}", self.map)
    }
}

impl ProcessList {
    pub fn new() -> Self {
        let mut new_list: BTreeMap<ProcessId, Process> = BTreeMap::new();

        let mut null_process: Process = Process::new(ProcessId::NULL_PROCESS);
        null_process.state = State::Current;
        null_process.kstack = Some(Vec::new());

        new_list.insert(ProcessId::NULL_PROCESS, null_process);

        ProcessList {
            map: new_list,
            next_id: 1,
        }
    }

    pub fn get(&self, id: ProcessId) -> Option<&Process> {
        self.map.get(&id)
    }

    pub fn get_mut(&mut self, id: ProcessId) -> Option<&mut Process> {
        self.map.get_mut(&id)
    }

    pub fn iter(&self) -> btree_map::Iter<ProcessId, Process> {
        self.map.iter()
    }

    pub fn add(&mut self) -> Result<&mut Process, Error> {
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
                    .insert(id, Process::new(id))
                    .is_none(),
                "Process id already exists!"
            );

            Ok(self.map.get_mut(&id).expect("Failed to add new process"))
        }
    }

    pub fn remove(&mut self, id: ProcessId) -> Option<Process> {
        self.map.remove(&id)
    }
}
