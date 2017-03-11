use self::paging::{PAGE_SIZE, PhysicalAddress};
pub use self::area_frame_allocator::AreaFrameAllocator;
pub use self::paging::remap_the_kernel;

mod area_frame_allocator;
pub mod paging;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    fn containing_address(address: usize) -> Frame {
        Frame { number: address / PAGE_SIZE }
    }

    fn clone(&self) -> Frame {
        Frame { number: self.number }
    }

    fn range_inclusive(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            start: start,
            end: end,
        }
    }

    fn start_address(&self) -> PhysicalAddress {
        self.number * PAGE_SIZE
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

struct FrameIter {
    start: Frame,
    end: Frame,
}

impl Iterator for FrameIter {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        if self.start <= self.end {
            let frame = self.start.clone();
            self.start.number += 1;
            Some(frame)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    #[should_panic]
    // Stack overflow test that could corrupt memory below stack
    // Issue: Use stack probes to check required stack pages before function
    // Tracking: https://github.com/rust-lang/rust/issues/16012
    fn stack_overflow() {
        let x = [0; 99999];
    }
}
