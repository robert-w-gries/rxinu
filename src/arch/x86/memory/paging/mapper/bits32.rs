use arch::x86::memory::{Frame, FrameAllocator};
use arch::x86::memory::paging::{PhysicalAddress, VirtualAddress, PAGE_SIZE};
use arch::x86::memory::paging::entry::EntryFlags;
use arch::x86::memory::paging::page::Page;
use arch::x86::memory::paging::mapper::Mapper;
use arch::x86::memory::paging::table::{self, Table, RECURSIVE_ENTRY};
use core::ptr::Unique;

pub type TableMapper = PageDirectoryMapper;

pub struct PageDirectoryMapper {
    p2: Unique<Table<table::Level2>>,
}

impl Mapper for PageDirectoryMapper {
    unsafe fn new() -> TableMapper {
        TableMapper {
            p2: Unique::new_unchecked(RECURSIVE_ENTRY),
        }
    }

    fn identity_map<A>(&mut self, frame: Frame, flags: EntryFlags, allocator: &mut A)
    where
        A: FrameAllocator,
    {
        let page = Page::containing_address(frame.start_address());
        self.map_to(page, frame, flags, allocator)
    }

    fn top_table(&self) -> &Table<table::Level2> {
        unsafe { self.p2.as_ref() }
    }

    fn top_table_mut(&mut self) -> &mut Table<table::Level2> {
        unsafe { self.p2.as_mut() }
    }

    fn map<A>(&mut self, page: Page, flags: EntryFlags, allocator: &mut A)
    where
        A: FrameAllocator,
    {
        let frame = allocator.allocate_frame().expect("out of memory");
        self.map_to(page, frame, flags, allocator)
    }

    fn map_to<A>(&mut self, page: Page, frame: Frame, flags: EntryFlags, allocator: &mut A)
    where
        A: FrameAllocator,
    {
        let p1 = self.top_table_mut()
            .next_table_create(page.p2_index(), allocator);

        assert!(p1[page.p1_index()].is_unused());
        p1[page.p1_index()].set(frame, flags | EntryFlags::PRESENT);
    }

    fn translate(&self, virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
        let offset = virtual_address % PAGE_SIZE;
        self.translate_page(Page::containing_address(virtual_address))
            .map(|frame| frame.number * PAGE_SIZE + offset)
    }

    fn translate_page(&self, page: Page) -> Option<Frame> {
        let p1 = self.top_table().next_table(page.p2_index());

        let huge_page = || {
            //let p2_entry = self.p2()[page.p2_index()];
            //// 4MiB page?
            //if let Some(start_frame) = p2_entry.pointed_frame() {
            //    if p2_entry.flags().contains(HUGE_PAGE) {
            //        // address must be 4MiB aligned
            //        assert!(start_frame.number % ENTRY_COUNT == 0);
            //        return Some(Frame { number: start_frame.number + page.p1_index() });
            //    }
            //}
            None
        };

        p1.and_then(|p1| p1[page.p1_index()].pointed_frame())
            .or_else(huge_page)
    }

    #[allow(unused)]
    fn unmap<A>(&mut self, page: Page, allocator: &mut A)
    where
        A: FrameAllocator,
    {
        assert!(self.translate(page.start_address()).is_some());

        let p1 = self.top_table_mut()
            .next_table_mut(page.p2_index())
            .expect("mapping code does not support huge pages");
        let frame = p1[page.p1_index()].pointed_frame().unwrap();
        p1[page.p1_index()].set_unused();
        unsafe {
            ::x86::shared::tlb::flush(page.start_address());
        }
        // TODO free p(1,2,3) table if empty
        //allocator.deallocate_frame(frame);
    }
}
