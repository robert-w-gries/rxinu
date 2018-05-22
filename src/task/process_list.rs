use alloc::btree_map::{self, BTreeMap};
use alloc::{String, Vec};
use arch::context::Context;
use core::fmt;
use core::result::Result;
use syscall::error::Error;
use task::{Priority, Process, ProcessId, Scheduling, State};

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

        let null_process = Process {
            pid: ProcessId::NULL_PROCESS,
            name: String::from("NULL"),
            state: State::Current,
            prio: Priority(0),
            context: Context::empty(),
            kstack: Some(Vec::new()),
        };

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

    pub fn add(&mut self, name: String, proc_entry: extern "C" fn(), sched_tobj: usize) -> Result<ProcessId, Error> {
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
                    .insert(id, Process::new(id, name, proc_entry, sched_tobj))
                    .is_none(),
                "Process id already exists!"
            );

            Ok(id)
        }
    }

    pub fn remove(&mut self, id: ProcessId) -> Option<Process> {
        self.map.remove(&id)
    }
}
