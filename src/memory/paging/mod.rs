use core::ops::{Deref, DerefMut};
use multiboot2::BootInformation;

use memory::{Frame, FrameAllocator};
use self::entry::{PRESENT, WRITABLE};
use self::mapper::Mapper;
use self::page::Page;
use self::temporary_page::TemporaryPage;

const ENTRY_COUNT: usize = 512;
pub const PAGE_SIZE: usize = 4096;

#[cfg(target_arch = "x86")]
const PHYS_ADDR_MASK: usize = 0x0;

#[cfg(target_arch = "x86_64")]
const PHYS_ADDR_MASK: usize = 0x000fffff_fffff000;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

mod entry;
mod mapper;
mod page;
mod table;
mod temporary_page;

pub struct ActivePageTable {
    mapper: Mapper,
}

impl Deref for ActivePageTable {
    type Target = Mapper;

    fn deref(&self) -> &Mapper {
        &self.mapper
    }
}

impl DerefMut for ActivePageTable {
    fn deref_mut(&mut self) -> &mut Mapper {
        &mut self.mapper
    }
}

impl ActivePageTable {
    unsafe fn new() -> ActivePageTable {
        ActivePageTable { mapper: Mapper::new() }
    }

    pub fn with<F>(&mut self,
                   table: &mut InactivePageTable,
                   temporary_page: &mut temporary_page::TemporaryPage, // new
                   f: F)
        where F: FnOnce(&mut Mapper)
    {
        use x86::shared::{control_regs, tlb};
        let flush_tlb = || unsafe { tlb::flush_all() };

        {
            let backup = Frame::containing_address(unsafe { control_regs::cr3() } as usize);

            // map temporary_page to current p4 table
            let p4_table = temporary_page.map_table_frame(backup.clone(), self);

            // overwrite recursive mapping
            self.p4_mut()[511].set(table.p4_frame.clone(), PRESENT | WRITABLE);
            flush_tlb();

            // execute f in the new context
            f(self);

            // restore recursive mapping to original p4 table
            p4_table[511].set(backup, PRESENT | WRITABLE);
            flush_tlb();
        }

        temporary_page.unmap(self);
    }
}

pub struct InactivePageTable {
    p4_frame: Frame,
}

impl InactivePageTable {
    pub fn new(frame: Frame,
               active_table: &mut ActivePageTable,
               temporary_page: &mut TemporaryPage)
               -> InactivePageTable {
        {
            let table = temporary_page.map_table_frame(frame.clone(), active_table);
            table.zero();
            // set up recursive mapping for the table
            table[511].set(frame.clone(), PRESENT | WRITABLE);
        }
        temporary_page.unmap(active_table);

        InactivePageTable { p4_frame: frame }
    }
}

pub fn test_paging<A>(allocator: &mut A)
    where A: FrameAllocator
{
    let page_table = unsafe { ActivePageTable::new() };

    // address 0 is mapped
    assert!(page_table.mapper.translate(0).is_some());

    // second P1 entry
    assert!(page_table.mapper.translate(4096).is_some());

    // second P2 entry
    assert!(page_table.mapper.translate(512 * 4096).is_some());

    // 300th P2 entry
    assert!(page_table.mapper.translate(300 * 512 * 4096).is_some());

    // second P3 entry
    assert!(page_table.mapper.translate(512 * 512 * 4096).is_none());

    // last mapped byte
    assert!(page_table.mapper.translate(512 * 512 * 4096 - 1).is_some());

}

pub fn remap_the_kernel<A>(allocator: &mut A, boot_info: &BootInformation)
    where A: FrameAllocator
{
    let mut temporary_page = TemporaryPage::new(Page { number: 0xdeadbeef }, allocator);

    let mut active_table = unsafe { ActivePageTable::new() };
    let mut new_table = {
        let frame = allocator.allocate_frame().expect("no more frames");
        InactivePageTable::new(frame, &mut active_table, &mut temporary_page)
    };

    active_table.with(&mut new_table, &mut temporary_page, |mapper| {
        let elf_sections_tag = boot_info.elf_sections_tag()
            .expect("Memory map tag required");

        for section in elf_sections_tag.sections() {
            use self::entry::WRITABLE;

            if !section.is_allocated() {
                // section is not loaded to memory
                continue;
            }
            assert!(section.addr as usize % PAGE_SIZE == 0,
                    "sections need to be page aligned");

            println!("mapping section at addr: {:#x}, size: {:#x}",
                     section.addr,
                     section.size);

            let flags = WRITABLE; // TODO use real section flags

            let start_frame = Frame::containing_address(section.start_address());
            let end_frame = Frame::containing_address(section.end_address() - 1);
            for frame in Frame::range_inclusive(start_frame, end_frame) {
                mapper.identity_map(frame, flags, allocator);
            }
        }
    });
}
