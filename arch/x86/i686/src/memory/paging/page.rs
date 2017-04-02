use super::{VirtualAddress, PAGE_SIZE};

#[derive(Debug, Clone, Copy)]
pub struct Page {
    pub number: usize,
}

impl Page {
    pub fn containing_address(address: VirtualAddress) -> Page {
        Page { number: address / PAGE_SIZE }
    }

    pub fn p2_index(&self) -> usize {
        (self.number >> 10) & 0x3ff
    }

    pub fn p1_index(&self) -> usize {
        (self.number >> 0) & 0x3ff
    }

    pub fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }
}
