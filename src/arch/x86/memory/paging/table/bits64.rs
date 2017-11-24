use super::{HierarchicalLevel, Level2, Table, TableLevel};

pub type TopLevelTable = Table<Level4>;

pub const RECURSIVE_ENTRY: *mut TopLevelTable = 0xffff_ffff_ffff_f000 as *mut _;

pub fn get_address(address: usize, index: usize) -> usize {
    (address << 9) | (index << 12)
}

pub enum Level4 {}
pub enum Level3 {}

impl TableLevel for Level4 {}
impl TableLevel for Level3 {}

impl HierarchicalLevel for Level4 {
    type NextLevel = Level3;
}

impl HierarchicalLevel for Level3 {
    type NextLevel = Level2;
}
