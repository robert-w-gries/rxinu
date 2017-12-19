use arch::x86::memory::{Frame, FrameAllocator};
use core::ops::{Deref, DerefMut};
use multiboot2::BootInformation;

use self::entry::EntryFlags;
use self::mapper::{Mapper, TableMapper};
use self::page::Page;
use self::temporary_page::TemporaryPage;

pub const PAGE_SIZE: usize = 4096;

#[cfg(target_arch = "x86")]
const ENTRY_COUNT: usize = 1024;
#[cfg(target_arch = "x86")]
const PHYS_ADDR_MASK: usize = 0xffff_f000;
#[cfg(target_arch = "x86")]
const TEMP_PAGE_ADDR: usize = 0xbeef;

#[cfg(target_arch = "x86_64")]
const ENTRY_COUNT: usize = 512;
#[cfg(target_arch = "x86_64")]
const PHYS_ADDR_MASK: usize = 0x000f_ffff_ffff_f000;
#[cfg(target_arch = "x86_64")]
const TEMP_PAGE_ADDR: usize = 0xdead_beef;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub mod entry;
pub mod mapper;
pub mod page;
mod table;
mod temporary_page;

pub struct ActivePageTable {
    mapper: TableMapper,
}

impl Deref for ActivePageTable {
    type Target = TableMapper;

    fn deref(&self) -> &TableMapper {
        &self.mapper
    }
}

impl DerefMut for ActivePageTable {
    fn deref_mut(&mut self) -> &mut TableMapper {
        &mut self.mapper
    }
}

impl ActivePageTable {
    pub unsafe fn new() -> ActivePageTable {
        ActivePageTable {
            mapper: TableMapper::new(),
        }
    }

    pub unsafe fn address(&self) -> usize {
        use x86::shared::control_regs;
        control_regs::cr3() as usize
    }

    pub fn switch(&mut self, new_table: InactivePageTable) -> InactivePageTable {
        use x86::shared::control_regs;

        let old_table = InactivePageTable {
            frame: Frame::containing_address(unsafe { control_regs::cr3() } as usize),
        };
        unsafe {
            control_regs::cr3_write(new_table.frame.start_address() as u64);
        }
        old_table
    }

    pub fn with<F>(
        &mut self,
        table: &mut InactivePageTable,
        temporary_page: &mut temporary_page::TemporaryPage, // new
        f: F,
    ) where
        F: FnOnce(&mut TableMapper),
    {
        use x86::shared::{control_regs, tlb};

        let flush_tlb = || unsafe { tlb::flush_all() };

        {
            let backup = Frame::containing_address(unsafe { control_regs::cr3() } as usize);

            // map temporary_page to current top table
            let top_table = temporary_page.map_table_frame(backup.clone(), self);

            // overwrite recursive mapping
            self.top_table_mut()[ENTRY_COUNT - 1].set(
                table.frame.clone(),
                EntryFlags::PRESENT | EntryFlags::WRITABLE,
            );
            flush_tlb();

            // execute f in the new context
            f(self);

            // restore recursive mapping to original top table
            top_table[ENTRY_COUNT - 1].set(backup, EntryFlags::PRESENT | EntryFlags::WRITABLE);
            flush_tlb();
        }

        temporary_page.unmap(self);
    }
}

pub struct InactivePageTable {
    frame: Frame,
}

impl InactivePageTable {
    pub fn new(
        frame: Frame,
        active_table: &mut ActivePageTable,
        temporary_page: &mut TemporaryPage,
    ) -> InactivePageTable {
        {
            let table = temporary_page.map_table_frame(frame.clone(), active_table);
            table.zero();
            // set up recursive mapping for the table
            table[ENTRY_COUNT - 1].set(frame.clone(), EntryFlags::PRESENT | EntryFlags::WRITABLE);
        }
        temporary_page.unmap(active_table);

        InactivePageTable { frame: frame }
    }
}

pub fn remap_the_kernel<A>(allocator: &mut A, boot_info: &BootInformation) -> ActivePageTable
where
    A: FrameAllocator,
{
    let mut temporary_page = TemporaryPage::new(
        Page {
            number: TEMP_PAGE_ADDR,
        },
        allocator,
    );

    let mut active_table = unsafe { ActivePageTable::new() };
    let mut new_table = {
        let frame = allocator.allocate_frame().expect("no more frames");
        InactivePageTable::new(frame, &mut active_table, &mut temporary_page)
    };

    active_table.with(&mut new_table, &mut temporary_page, |mapper| {
        let elf_sections_tag = boot_info
            .elf_sections_tag()
            .expect("Memory map tag required");

        for section in elf_sections_tag.sections() {
            if !section.is_allocated() {
                // section is not loaded to memory
                continue;
            }
            assert!(
                section.start_address() as usize % PAGE_SIZE == 0,
                "sections need to be page aligned"
            );

            kprintln!(
                "mapping section at addr: {:#x}, size: {:#x}",
                section.start_address(),
                section.size()
            );

            let flags = EntryFlags::from_elf_section_flags(&section);

            let start_frame = Frame::containing_address(section.start_address() as usize);
            let end_frame = Frame::containing_address(section.end_address() as usize - 1);
            for frame in Frame::range_inclusive(start_frame, end_frame) {
                mapper.identity_map(frame, flags, allocator);
            }
        }

        // identity map the VGA text buffer
        let vga_buffer_frame = Frame::containing_address(0xb8000);
        mapper.identity_map(vga_buffer_frame, EntryFlags::WRITABLE, allocator);

        let multiboot_start = Frame::containing_address(boot_info.start_address());
        let multiboot_end = Frame::containing_address(boot_info.end_address() - 1);
        for frame in Frame::range_inclusive(multiboot_start, multiboot_end) {
            mapper.identity_map(frame, EntryFlags::PRESENT, allocator);
        }
    });

    let old_table = active_table.switch(new_table);

    // turn the old top page into a guard page
    let old_top_page = Page::containing_address(old_table.frame.start_address());
    active_table.unmap(old_top_page, allocator);
    kprintln!("guard page at {:#x}", old_top_page.start_address());
    active_table
}
