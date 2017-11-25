enum State {
    Free,
    Current,
    Suspended,
    Ready,
}

#[derive(Ord)]
struct ProcessId(u64);

impl Add for ProcessId {
    type Output = ProcessId;

    fn add(self, other: ProcessId) -> ProcessId {
        ProcessId { self.0 + other.0 }
    }
}

pub struct Process {
    pid: ProcessId,
    state: State,
    stack_pointer: usize,
    stack_base: usize,
    stack_length: usize,
    name: String,
}

impl Process {
    pub const NEW: Process = Process {
        pid: ProcessId(0),
        state: State::Suspended,
        stack_pointer: 0,
        stack_base: 0,
        stack_length: 0,
        name: "NEW",
    };
}

pub struct ProcessList<T> {
    collection: T,
    next_id: ProcessId,
}

impl ProcessList<BTreeMap> {
    pub fn new() -> Self {
        ProcessList {
            map: BTreeMap::new(),
            next_id: 1,
        }
    }

    pub fn get(&self, id: ProcessId) -> Option<Process> {
        self.map.get(&id)
    }

    pub fn current(&self) -> Option<Process> {
        self.map.get(*CURRENT_PID.lock())
    }

    pub fn iter(&self) -> btree_map::Iter<ProcessId, Process> {
        self.map.iter()
    }

    pub fn add(&mut self) -> Result<Process> {
    }
}
