use arch::x86::memory::FrameAllocator;
use os_bootinfo::{FrameRange, MemoryMap, MemoryRegionType};
use x86_64::structures::paging::{PhysFrame, PhysFrameRange, Size4KB};

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

impl FrameAllocator for AreaFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let region = &mut self.memory_map
            .iter_mut()
            .filter(|region| region.region_type == MemoryRegionType::Usable)
            .next();

        let frame_range: &mut FrameRange = &mut region
            .as_mut()
            .expect("Could not find usable memory region")
            .range;

        let mut phys_range = PhysFrameRange::<Size4KB>::from(*frame_range);

        if let Some(frame) = phys_range.next() {
            frame_range.start_frame_number =
                phys_range.start.start_address().as_u64() / frame.size();
            Some(frame)
        } else {
            None
        }
    }

    #[allow(unused)]
    fn deallocate_frame(&mut self, frame: PhysFrame) {
        unimplemented!()
    }
}
