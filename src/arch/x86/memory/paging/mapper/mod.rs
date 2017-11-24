use arch::x86::memory::{Frame, FrameAllocator};

use super::{PhysicalAddress, VirtualAddress};
use super::entry::EntryFlags;
use super::page::Page;
use super::table::TopLevelTable;

#[cfg(target_arch = "x86")]
mod bits32;
#[cfg(target_arch = "x86_64")]
mod bits64;

#[cfg(target_arch = "x86")]
pub use self::bits32::*;
#[cfg(target_arch = "x86_64")]
pub use self::bits64::*;

pub trait Mapper {
    unsafe fn new() -> TableMapper;
    fn identity_map<A: FrameAllocator>(&mut self, frame: Frame, flags: EntryFlags, allocator: &mut A);
    fn map<A: FrameAllocator>(&mut self, page: Page, flags: EntryFlags, allocator: &mut A);
    fn map_to<A: FrameAllocator>(&mut self, page: Page, frame: Frame, flags: EntryFlags, allocator: &mut A);
    fn top_table(&self) -> &TopLevelTable;
    fn top_table_mut(&mut self) -> &mut TopLevelTable;
    fn translate(&self, virtual_address: VirtualAddress) -> Option<PhysicalAddress>;
    fn translate_page(&self, page: Page) -> Option<Frame>;
    fn unmap<A: FrameAllocator>(&mut self, page: Page, allocator: &mut A);
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
        assert_eq!(
            allocator.allocate_frame().expect("no more frames").number,
            3
        );
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
