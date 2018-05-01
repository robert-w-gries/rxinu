use os_bootinfo::{FrameRange, MemoryMap, MemoryRegion, MemoryRegionType};
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

use arch::x86::memory::FrameAllocator;

impl FrameAllocator for AreaFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let regions: &mut [MemoryRegion] = &mut *self.memory_map;

        let region = &mut regions
            .iter_mut()
            .filter(|region| region.region_type == MemoryRegionType::Usable)
            .next();

        let frame_range: &mut FrameRange = &mut region
            .as_mut()
            .expect("Could not find usable memory region")
            .range;

        let mut range = PhysFrameRange::<Size4KB>::from(*frame_range);

        let frame: Option<PhysFrame> = range.next();

        if let Some(f) = frame {
            frame_range.start_frame_number = range.start.start_address().as_u64() / f.size();
        }

        frame
    }

    #[allow(unused)]
    fn deallocate_frame(&mut self, frame: PhysFrame) {
        unimplemented!()
    }
}
