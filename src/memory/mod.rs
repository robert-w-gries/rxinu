use self::paging::{PAGE_SIZE, PhysicalAddress};
pub use self::area_frame_allocator::AreaFrameAllocator;

pub use self::paging::mapper::test_paging;

mod area_frame_allocator;
pub mod paging;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
        number: usize,
}

impl Frame {
    fn containing_address(address: usize) -> Frame {
        Frame{ number: address / PAGE_SIZE }
    }

	fn start_address(&self) -> PhysicalAddress {
	    self.number * PAGE_SIZE
	}
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}
