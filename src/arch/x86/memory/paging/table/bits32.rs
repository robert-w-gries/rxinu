use super::{Level2, Table};

pub type TopLevelTable = Table<Level2>;

pub const RECURSIVE_ENTRY: *mut TopLevelTable = 0xffff_f000 as *mut _;

pub fn get_address(address: usize, index: usize) -> usize {
    (address << 10) | (index << 12)
}
