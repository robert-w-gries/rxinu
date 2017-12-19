use multiboot2::ElfSection;

use arch::x86::memory::Frame;
use arch::x86::memory::paging::PHYS_ADDR_MASK;

#[cfg(target_arch = "x86")]
type EntryBits = u32;
#[cfg(target_arch = "x86_64")]
type EntryBits = u64;

pub struct Entry(EntryBits);

impl Entry {
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }

    pub fn set_unused(&mut self) {
        self.0 = 0;
    }

    pub fn flags(&self) -> EntryFlags {
        EntryFlags::from_bits_truncate(self.0)
    }

    pub fn pointed_frame(&self) -> Option<Frame> {
        if self.flags().contains(EntryFlags::PRESENT) {
            Some(Frame::containing_address(self.0 as usize & PHYS_ADDR_MASK))
        } else {
            None
        }
    }

    pub fn set(&mut self, frame: Frame, flags: EntryFlags) {
        assert!((frame.start_address() & !PHYS_ADDR_MASK) == 0);
        self.0 = (frame.start_address() as EntryBits) | flags.bits();
    }
}

impl EntryFlags {
    #[cfg(target_arch = "x86")]
    pub fn from_elf_section_flags(section: &ElfSection) -> EntryFlags {
        use multiboot2::ElfSectionFlags;

        let mut flags = EntryFlags::empty();

        if section.flags().contains(ElfSectionFlags::ALLOCATED) {
            // section is loaded to memory
            flags = flags | EntryFlags::PRESENT;
        }
        if section.flags().contains(ElfSectionFlags::WRITABLE) {
            flags = flags | EntryFlags::WRITABLE;
        }
        flags
    }

    #[cfg(target_arch = "x86_64")]
    pub fn from_elf_section_flags(section: &ElfSection) -> EntryFlags {
        use multiboot2::ElfSectionFlags;

        let mut flags = EntryFlags::empty();

        if section.flags().contains(ElfSectionFlags::ALLOCATED) {
            // section is loaded to memory
            flags = flags | EntryFlags::PRESENT;
        }
        if section.flags().contains(ElfSectionFlags::WRITABLE) {
            flags = flags | EntryFlags::WRITABLE;
        }
        if !section.flags().contains(ElfSectionFlags::EXECUTABLE) {
            flags = flags | EntryFlags::NO_EXECUTE;
        }
        flags
    }
}

#[cfg(target_arch = "x86")]
bitflags! {
    pub struct EntryFlags: u32 {
        const PRESENT =         1 << 0;
        const WRITABLE =        1 << 1;
        const USER_ACCESSIBLE = 1 << 2;
        const WRITE_THROUGH =   1 << 3;
        const NO_CACHE =        1 << 4;
        const ACCESSED =        1 << 5;
        const DIRTY =           1 << 6;
        const HUGE_PAGE =       1 << 7;
        const GLOBAL =          1 << 8;
    }
}

#[cfg(target_arch = "x86_64")]
bitflags! {
    pub struct EntryFlags: u64 {
        const PRESENT =         1 << 0;
        const WRITABLE =        1 << 1;
        const USER_ACCESSIBLE = 1 << 2;
        const WRITE_THROUGH =   1 << 3;
        const NO_CACHE =        1 << 4;
        const ACCESSED =        1 << 5;
        const DIRTY =           1 << 6;
        const HUGE_PAGE =       1 << 7;
        const GLOBAL =          1 << 8;
        const NO_EXECUTE =      1 << 63;
    }
}
