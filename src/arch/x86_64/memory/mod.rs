use bootloader::bootinfo::BootInfo;
use x86_64::structures::paging::{
    mapper::MapToError, FrameAllocator, MappedPageTable, Mapper, Page, PageTable, PageTableFlags,
    PhysFrame, Size4KiB,
};
use x86_64::VirtAddr;

pub use self::area_frame_allocator::AreaFrameAllocator;
pub use self::stack_allocator::Stack;

mod area_frame_allocator;
pub mod heap;
mod stack_allocator;

pub unsafe fn init(boot_info: &'static BootInfo) {
    let mut mapper = {
        let physical_memory_offset = boot_info.physical_memory_offset;
        let level_4_table = active_level_4_table(physical_memory_offset);
        let phys_to_virt = move |frame: PhysFrame| -> *mut PageTable {
            let phys = frame.start_address().as_u64();
            let virt = VirtAddr::new(phys + physical_memory_offset);
            virt.as_mut_ptr()
        };
        MappedPageTable::new(level_4_table, phys_to_virt)
    };

    let mut frame_allocator = area_frame_allocator::init_frame_allocator(&boot_info.memory_map);

    use self::heap::{HEAP_SIZE, HEAP_START};

    let heap_start_page = Page::containing_address(VirtAddr::new(HEAP_START));
    let heap_end_page = Page::containing_address(VirtAddr::new(HEAP_START + HEAP_SIZE - 1));

    for page in Page::range_inclusive(heap_start_page, heap_end_page) {
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::NO_EXECUTE;
        map_page(&mut mapper, page, flags, &mut frame_allocator).expect("Heap page mapping failed");
    }

    let _stack_allocator = {
        let stack_alloc_start = heap_end_page + 1;
        let stack_alloc_end = stack_alloc_start + 100;
        let stack_alloc_range = Page::range_inclusive(stack_alloc_start, stack_alloc_end);
        stack_allocator::StackAllocator::new(stack_alloc_range)
    };
}

pub fn map_page<'a, A>(
    mapper: &mut impl Mapper<Size4KiB>,
    page: Page<Size4KiB>,
    flags: PageTableFlags,
    frame_allocator: &mut A,
) -> Result<(), MapToError>
where
    A: FrameAllocator<Size4KiB>,
{
    let frame = frame_allocator
        .allocate_frame()
        .expect("OOM - Cannot allocate frame");

    unsafe {
        mapper.map_to(page, frame, flags, frame_allocator)?.flush();
    }

    Ok(())
}

/// Returns a mutable reference to the active level 4 table.
///
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn active_level_4_table(physical_memory_offset: u64) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = VirtAddr::new(phys.as_u64() + physical_memory_offset);
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}
