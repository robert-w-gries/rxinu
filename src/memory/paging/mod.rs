use core::ptr::Unique;

use self::mapper::Mapper;
use self::page::Page;
use self::table::{Table, Level4};

const ENTRY_COUNT: usize = 512;
const PHYS_ADDR_MASK: usize = 0x000fffff_fffff000;
pub const PAGE_SIZE: usize = 4096;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

mod entry;
pub mod mapper;
mod page;
mod table;

pub struct ActivePageTable {
	mapper: Mapper,
}

