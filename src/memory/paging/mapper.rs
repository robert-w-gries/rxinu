use core::ptr::Unique;

use memory::{Frame, FrameAllocator};
use super::{PhysicalAddress, VirtualAddress, ENTRY_COUNT, PAGE_SIZE};
use super::entry::{EntryFlags, PRESENT, HUGE_PAGE};
use super::page::Page;
use super::table::{self, Table, Level4, P4};

pub struct Mapper {
    p4: Unique<Table<Level4>>,
}

impl Mapper {
    pub unsafe fn new() -> Mapper {
        Mapper { p4: Unique::new(P4) }
    }

	pub fn identity_map<A>(&mut self,
	                       frame: Frame,
	                       flags: EntryFlags,
	                       allocator: &mut A)
	    where A: FrameAllocator
	{
	    let page = Page::containing_address(frame.start_address());
	    self.map_to(page, frame, flags, allocator)
	}

    fn p4(&self) -> &Table<Level4> {
        unsafe { self.p4.get() }
    }

    fn p4_mut(&mut self) -> &mut Table<Level4> {
        unsafe { self.p4.get_mut() }
    }

    pub fn map_to<A>(&mut self, page: Page, frame: Frame, flags: EntryFlags,
                 allocator: &mut A)
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

	pub fn translate(&self, virtual_address: VirtualAddress) -> Option<PhysicalAddress>
	{
	    let offset = virtual_address % PAGE_SIZE;
	    self.translate_page(Page::containing_address(virtual_address))
	        .map(|frame| frame.number * PAGE_SIZE + offset)
	}

    pub fn translate_page(&self, page: Page) -> Option<Frame> {
        let p3 = unsafe { &*P4 }.next_table(page.p4_index());

        let huge_page = || {
            p3.and_then(|p3| {
                  let p3_entry = &p3[page.p3_index()];
                  // 1GiB page?
                  if let Some(start_frame) = p3_entry.pointed_frame() {
                      if p3_entry.flags().contains(HUGE_PAGE) {
                          // address must be 1GiB aligned
                          assert!(start_frame.number % (ENTRY_COUNT * ENTRY_COUNT) == 0);
                          return Some(Frame {
                              number: start_frame.number + page.p2_index() *
                                      ENTRY_COUNT + page.p1_index(),
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
                                  number: start_frame.number + page.p1_index()
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

	fn unmap<A>(&mut self, page: Page, allocator: &mut A)
	    where A: FrameAllocator
	{
	    assert!(self.translate(page.start_address()).is_some());

	    let p1 = self.p4_mut()
	                 .next_table_mut(page.p4_index())
	                 .and_then(|p3| p3.next_table_mut(page.p3_index()))
	                 .and_then(|p2| p2.next_table_mut(page.p2_index()))
	                 .expect("mapping code does not support huge pages");
	    let frame = p1[page.p1_index()].pointed_frame().unwrap();
	    p1[page.p1_index()].set_unused();
	    // TODO free p(1,2,3) table if empty
	    allocator.deallocate_frame(frame);
	}
}

    pub fn test_paging<A>(allocator: &mut A)
        where A: FrameAllocator
    {
        let page_table = unsafe { Mapper::new() };

        // address 0 is mapped
//        assert_eq!(page_table.translate(0).is_some(), true);

         // second P1 entry
        assert_eq!(page_table.translate(4096).is_some(), true);

        // second P2 entry
        assert_eq!(page_table.translate(512 * 4096).is_some(), true);

        // 300th P2 entry
        assert_eq!(page_table.translate(300 * 512 * 4096).is_some(), true);

        // second P3 entry
        assert_eq!(page_table.translate(512 * 512 * 4096).is_none(), true);

        // last mapped byte
        assert_eq!(page_table.translate(512 * 512 * 4096 - 1).is_some(), true);

    }


#[cfg(test)]
mod tests {
	use super::Mapper;

	#[test]
	fn test_paging() {
		let page_table = unsafe { Mapper::new() };

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
}
