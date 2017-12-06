use core::cmp;
use core::sync::atomic::{self, AtomicUsize};

pub struct ProcessId(AtomicUsize);

impl PartialEq for ProcessId {
    fn eq(&self, other: &ProcessId) -> bool {
        self.get_usize() == other.get_usize()
    }
}

impl Eq for ProcessId {}

impl PartialOrd for ProcessId {
    fn partial_cmp(&self, other: &ProcessId) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ProcessId {
    fn cmp(&self, other: &ProcessId) -> cmp::Ordering {
        self.get_usize().cmp(&other.get_usize())
    }
}

impl Clone for ProcessId {
    fn clone(&self) -> Self {
        ProcessId::new(self.get_usize())
    }
}

impl ProcessId {
    pub const NULL_PROCESS: ProcessId = ProcessId(AtomicUsize::new(0));

    pub fn new(id: usize) -> ProcessId {
        ProcessId(AtomicUsize::new(id))
    }

    pub fn set(&self, new_id: ProcessId) {
        self.set_usize(new_id.get_usize());
    }

    fn get_usize(&self) -> usize {
        self.0.load(atomic::Ordering::SeqCst)
    }

    fn set_usize(&self, id: usize) {
        self.0.store(id, atomic::Ordering::SeqCst);
    }
}
