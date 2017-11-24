use arch::x86::memory::{Frame, FrameAllocator};
use arch::x86::memory::paging::table::{self, TopLevelTable};
use arch::x86::memory::paging::{PhysicalAddress, VirtualAddress, PAGE_SIZE};
use arch::x86::memory::paging::entry::{EntryFlags, PRESENT};
use arch::x86::memory::paging::page::Page;
use core::ptr::Unique;
use super::Mapper;

pub type TableMapper = PageMapLevel4Mapper;

pub struct PageMapLevel4Mapper {
    p4: Unique<TopLevelTable>,
}

impl Mapper for PageMapLevel4Mapper {
    unsafe fn new() -> TableMapper {
        PageMapLevel4Mapper {
            p4: Unique::new_unchecked(table::RECURSIVE_ENTRY),
        }
    }

    fn identity_map<A>(&mut self, frame: Frame, flags: EntryFlags, allocator: &mut A)
    where
        A: FrameAllocator,
    {
        let page = Page::containing_address(frame.start_address());
        self.map_to(page, frame, flags, allocator)
    }

    fn top_table(&self) -> &TopLevelTable {
        unsafe { self.p4.as_ref() }
    }

    fn top_table_mut(&mut self) -> &mut TopLevelTable {
        unsafe { self.p4.as_mut() }
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
        let p3 = self.top_table_mut()
            .next_table_create(page.p4_index(), allocator);
        let p2 = p3.next_table_create(page.p3_index(), allocator);
        let p1 = p2.next_table_create(page.p2_index(), allocator);

        assert!(p1[page.p1_index()].is_unused());
        p1[page.p1_index()].set(frame, flags | PRESENT);
    }

    fn translate(&self, virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
        let offset = virtual_address % PAGE_SIZE;
        self.translate_page(Page::containing_address(virtual_address))
            .map(|frame| frame.number * PAGE_SIZE + offset)
    }

    fn translate_page(&self, page: Page) -> Option<Frame> {
        use arch::x86::memory::paging::ENTRY_COUNT;
        use arch::x86::memory::paging::entry::HUGE_PAGE;
        let p3 = self.top_table().next_table(page.p4_index());

        let huge_page = || {
            p3.and_then(|p3| {
                let p3_entry = &p3[page.p3_index()];
                // 1GiB page?
                if let Some(start_frame) = p3_entry.pointed_frame() {
                    if p3_entry.flags().contains(HUGE_PAGE) {
                        // address must be 1GiB aligned
                        assert!(start_frame.number % (ENTRY_COUNT * ENTRY_COUNT) == 0);
                        return Some(Frame {
                            number: start_frame.number + page.p2_index() * ENTRY_COUNT
                                + page.p1_index(),
                        });
                    }
                }
                if let Some(p2) = p3.next_table(page.p3_index()) {
                    let p2_entry = &p2[page.p2_index()];
                    // 2MiB page?
                    if let Some(start_frame) = p2_entry.pointed_frame() {
                        if p2_entry.flags().contains(HUGE_PAGE) {
                            // address must be 2MiB aligned
                            assert!(start_frame.number % ENTRY_COUNT == 0);
                            return Some(Frame {
                                number: start_frame.number + page.p1_index(),
                            });
                        }
                    }
                }
                None
            })
        };

        p3.and_then(|p3| p3.next_table(page.p3_index()))
            .and_then(|p2| p2.next_table(page.p2_index()))
            .and_then(|p1| p1[page.p1_index()].pointed_frame())
            .or_else(huge_page)
    }

    #[allow(unused)]
    fn unmap<A>(&mut self, page: Page, allocator: &mut A)
    where
        A: FrameAllocator,
    {
        assert!(self.translate(page.start_address()).is_some());

        let p1 = self.top_table_mut()
            .next_table_mut(page.p4_index())
            .and_then(|p3| p3.next_table_mut(page.p3_index()))
            .and_then(|p2| p2.next_table_mut(page.p2_index()))
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
