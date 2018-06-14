use os_bootinfo::{FrameRange, MemoryMap, MemoryRegionType};
use x86_64::structures::paging::{
    FrameAllocator, FrameDeallocator, PhysFrame, PhysFrameRange, Size4KiB,
};

pub struct AreaFrameAllocator {
    memory_map: MemoryMap,
}

impl AreaFrameAllocator {
    pub fn new(memory_map: &MemoryMap) -> Self {
        let mut mm = MemoryMap::new();
        for reg in memory_map.iter() {
            mm.add_region(reg.clone());
        }

        AreaFrameAllocator { memory_map: mm }
    }
}

impl FrameAllocator<Size4KiB> for AreaFrameAllocator {
    fn alloc(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let region = &mut self
            .memory_map
            .iter_mut()
            .filter(|region| region.region_type == MemoryRegionType::Usable)
            .next();

        let frame_range: &mut FrameRange = &mut region
            .as_mut()
            .expect("Could not find usable memory region")
            .range;

        let mut phys_range = PhysFrameRange::<Size4KiB>::from(*frame_range);

        if let Some(frame) = phys_range.next() {
            frame_range.start_frame_number =
                phys_range.start.start_address().as_u64() / frame.size();
            Some(frame)
        } else {
            None
        }
    }
}

impl FrameDeallocator<Size4KiB> for AreaFrameAllocator {
    #[allow(unused)]
    fn dealloc(&mut self, frame: PhysFrame<Size4KiB>) {
        unimplemented!()
    }
}
