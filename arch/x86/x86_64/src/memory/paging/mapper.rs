use core::ptr::Unique;

use memory::{Frame, FrameAllocator};
use super::{PhysicalAddress, VirtualAddress, ENTRY_COUNT, PAGE_SIZE};
use super::entry::{EntryFlags, PRESENT, HUGE_PAGE};
use super::page::Page;
use super::table::{self, Table, Level4};

pub struct Mapper {
    p4: Unique<Table<Level4>>,
}

impl Mapper {
    pub unsafe fn new() -> Mapper {
        Mapper { p4: Unique::new(table::P4) }
    }

    pub fn identity_map<A>(&mut self, frame: Frame, flags: EntryFlags, allocator: &mut A)
        where A: FrameAllocator
    {
        let page = Page::containing_address(frame.start_address());
        self.map_to(page, frame, flags, allocator)
    }

    pub fn p4(&self) -> &Table<Level4> {
        unsafe { self.p4.get() }
    }

    pub fn p4_mut(&mut self) -> &mut Table<Level4> {
        unsafe { self.p4.get_mut() }
    }

    pub fn map_to<A>(&mut self, page: Page, frame: Frame, flags: EntryFlags, allocator: &mut A)
        where A: FrameAllocator
    {
        let mut p3 = self.p4_mut().next_table_create(page.p4_index(), allocator);
        let mut p2 = p3.next_table_create(page.p3_index(), allocator);
        let mut p1 = p2.next_table_create(page.p2_index(), allocator);

        assert!(p1[page.p1_index()].is_unused());
        p1[page.p1_index()].set(frame, flags | PRESENT);
    }

    pub fn map<A>(&mut self, page: Page, flags: EntryFlags, allocator: &mut A)
        where A: FrameAllocator
    {
        let frame = allocator.allocate_frame().expect("out of memory");
        self.map_to(page, frame, flags, allocator)
    }

    pub fn translate(&self, virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
        let offset = virtual_address % PAGE_SIZE;
        self.translate_page(Page::containing_address(virtual_address))
            .map(|frame| frame.number * PAGE_SIZE + offset)
    }

    pub fn translate_page(&self, page: Page) -> Option<Frame> {
        let p3 = self.p4().next_table(page.p4_index());

        let huge_page = || {
            p3.and_then(|p3| {
                let p3_entry = &p3[page.p3_index()];
                // 1GiB page?
                if let Some(start_frame) = p3_entry.pointed_frame() {
                    if p3_entry.flags().contains(HUGE_PAGE) {
                        // address must be 1GiB aligned
                        assert!(start_frame.number % (ENTRY_COUNT * ENTRY_COUNT) == 0);
                        return Some(Frame {
                            number: start_frame.number + page.p2_index() * ENTRY_COUNT +
                                    page.p1_index(),
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
                            return Some(Frame { number: start_frame.number + page.p1_index() });
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
    pub fn unmap<A>(&mut self, page: Page, allocator: &mut A)
        where A: FrameAllocator
    {
        use x86_64::VirtualAddress;
        use x86_64::instructions::tlb;

        assert!(self.translate(page.start_address()).is_some());

        let p1 = self.p4_mut()
            .next_table_mut(page.p4_index())
            .and_then(|p3| p3.next_table_mut(page.p3_index()))
            .and_then(|p2| p2.next_table_mut(page.p2_index()))
            .expect("mapping code does not support huge pages");
        let frame = p1[page.p1_index()].pointed_frame().unwrap();
        p1[page.p1_index()].set_unused();
        tlb::flush(VirtualAddress(page.start_address()));
        // TODO free p(1,2,3) table if empty
        //allocator.deallocate_frame(frame);
    }
}

#[cfg(test)]
mod tests {
    use super::Mapper;

    #[test]
    fn map() {
        use memory::paging::page::Page;
        use memory::paging::entry::EntryFlags;

        let page_table = unsafe { Mapper::new() };

        let addr = 42 * 512 * 512 * 4096; // 42th P3 entry
        let page = Page::containing_address(addr);
        let frame = allocator.allocate_frame().expect("no more frames");

        // Test: Unmapped page should not have a physical address
        assert!(page_table.translate(addr).is_none());

        // Test: First frame returned by frame allocator is 0
        assert_eq!(frame.number, 0);

        // Test: Map page to Some frame
        page_table.map_to(page, frame, EntryFlags::empty(), allocator);
        assert!(page_table.translate(addr).is_some());

        // Test: Mapping code needs to create a P2 and P1 table
        //       Next returned frame is frame 3
        assert_eq!(allocator.allocate_frame().expect("no more frames").number,
                   3);
    }

    #[test]
    fn translate() {
        let mut page_table = unsafe { Mapper::new() };

        // address 0 is mapped
        assert!(page_table.translate(0).is_some());

        // second P1 entry
        assert!(page_table.translate(4096).is_some());

        // second P2 entry
        assert!(page_table.translate(512 * 4096).is_some());

        // 300th P2 entry
        assert!(page_table.translate(300 * 512 * 4096).is_some());

        // second P3 entry
        assert!(page_table.translate(512 * 512 * 4096).is_none());

        // last mapped byte
        assert!(page_table.translate(512 * 512 * 4096 - 1).is_some());

    }

    #[test]
    fn unmap() {
        use memory::paging::page::Page;
        use memory::paging::entry::EntryFlags;

        let page_table = unsafe { Mapper::new() };

        let addr = 42 * 512 * 512 * 4096; // 42th P3 entry
        let page = Page::containing_address(addr);
        let frame = allocator.allocate_frame().expect("no more frames");

        page_table.translate(addr).is_none();
        page_table.map_to(page, frame, EntryFlags::empty(), allocator);

        page_table.unmap(Page::containing_address(addr), allocator);
        assert!(page_table.translate(addr).is_none());
    }
}
