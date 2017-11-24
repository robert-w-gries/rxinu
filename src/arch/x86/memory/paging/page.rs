use core::ops::Add;
use super::{VirtualAddress, PAGE_SIZE};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Page {
    pub number: usize,
}

impl Page {
    pub fn containing_address(address: VirtualAddress) -> Page {
        #[cfg(target_arch = "x86_64")]
        assert!(
            address < 0x0000_8000_0000_0000 || address >= 0xffff_8000_0000_0000,
            "invalid address: 0x{:x}",
            address
        );
        Page {
            number: address / PAGE_SIZE,
        }
    }

    pub fn range_inclusive(start: Page, end: Page) -> PageIter {
        PageIter {
            start: start,
            end: end,
        }
    }

    pub fn start_address(&self) -> usize {
        self.number * PAGE_SIZE
    }
}

#[cfg(target_arch = "x86")]
impl Page {
    pub fn p2_index(&self) -> usize {
        (self.number >> 10) & 0x3ff
    }

    pub fn p1_index(&self) -> usize {
        (self.number >> 0) & 0x3ff
    }
}

#[cfg(target_arch = "x86_64")]
impl Page {
    pub fn p4_index(&self) -> usize {
        (self.number >> 27) & 0o777
    }

    pub fn p3_index(&self) -> usize {
        (self.number >> 18) & 0o777
    }

    pub fn p2_index(&self) -> usize {
        (self.number >> 9) & 0o777
    }

    pub fn p1_index(&self) -> usize {
        (self.number >> 0) & 0o777
    }
}

impl Add<usize> for Page {
    type Output = Page;

    fn add(self, rhs: usize) -> Page {
        Page {
            number: self.number + rhs,
        }
    }
}

#[derive(Clone)]
pub struct PageIter {
    start: Page,
    end: Page,
}

impl Iterator for PageIter {
    type Item = Page;

    fn next(&mut self) -> Option<Page> {
        if self.start <= self.end {
            let page = self.start;
            self.start.number += 1;
            Some(page)
        } else {
            None
        }
    }
}
