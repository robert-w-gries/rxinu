use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::structures::paging::{FrameAllocator, FrameDeallocator, PhysFrame, Size4KiB};
use x86_64::PhysAddr;

pub struct AreaFrameAllocator<I>
where
    I: Iterator<Item = PhysFrame>,
{
    frames: I,
}

pub fn init_frame_allocator(
    memory_map: &'static MemoryMap,
) -> AreaFrameAllocator<impl Iterator<Item = PhysFrame>> {
    let regions = memory_map
        .iter()
        .filter(|r| r.region_type == MemoryRegionType::Usable);

    // map each region to its address range
    let addr_ranges = regions.map(|r| r.range.start_addr()..r.range.end_addr());

    // transform to an iterator of frame start addresses
    let frame_addresses = addr_ranges.flat_map(|r| r.into_iter().step_by(4096));

    // create `PhysFrame` types from the start addresses
    let frames = frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)));

    AreaFrameAllocator { frames }
}

impl<I> FrameAllocator<Size4KiB> for AreaFrameAllocator<I>
where
    I: Iterator<Item = PhysFrame>,
{
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        self.frames.next()
    }
}

impl<I> FrameDeallocator<Size4KiB> for AreaFrameAllocator<I>
where
    I: Iterator<Item = PhysFrame>,
{
    #[allow(unused)]
    fn deallocate_frame(&mut self, frame: PhysFrame<Size4KiB>) {
        unimplemented!()
    }
}
