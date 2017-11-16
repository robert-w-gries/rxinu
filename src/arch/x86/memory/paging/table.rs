use arch::x86::memory::FrameAllocator;
use core::marker::PhantomData;
use core::ops::{Index, IndexMut};

use super::ENTRY_COUNT;
use super::entry::{Entry, HUGE_PAGE, PRESENT, WRITABLE};

#[cfg(target_arch = "x86")]
pub const P2: *mut Table<Level2> = 0xffff_f000 as *mut _;

#[cfg(target_arch = "x86_64")]
pub const P4: *mut Table<Level4> = 0xffff_ffff_ffff_f000 as *mut _;

pub struct Table<L: TableLevel> {
    entries: [Entry; ENTRY_COUNT],
    level: PhantomData<L>,
}

impl<L> Table<L>
    where L: TableLevel
{
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.set_unused();
        }
    }
}

impl<L> Table<L>
    where L: HierarchicalLevel
{
    pub fn next_table(&self, index: usize) -> Option<&Table<L::NextLevel>> {
        self.next_table_address(index)
            .map(|address| unsafe { &*(address as *const _) })
    }

    pub fn next_table_mut(&mut self, index: usize) -> Option<&mut Table<L::NextLevel>> {
        self.next_table_address(index)
            .map(|address| unsafe { &mut *(address as *mut _) })
    }

    pub fn next_table_create<A>(&mut self,
                                index: usize,
                                allocator: &mut A)
                                -> &mut Table<L::NextLevel>
        where A: FrameAllocator
    {
        if self.next_table(index).is_none() {
            assert!(!self.entries[index].flags().contains(HUGE_PAGE),
                    "mapping code does not support huge pages");
            let frame = allocator.allocate_frame().expect("no frames available");
            self.entries[index].set(frame, PRESENT | WRITABLE);
            self.next_table_mut(index).unwrap().zero();
        }
        self.next_table_mut(index).unwrap()
    }

    fn next_table_address(&self, index: usize) -> Option<usize> {
        let entry_flags = self[index].flags();
        if entry_flags.contains(PRESENT) && !entry_flags.contains(HUGE_PAGE) {
            let table_address = self as *const _ as usize;

            if cfg!(target_arch = "x86") {
                Some((table_address << 10) | (index << 12))
            } else {
                Some((table_address << 9) | (index << 12))
            }
        } else {
            None
        }
    }
}

impl<L> Index<usize> for Table<L>
    where L: TableLevel
{
    type Output = Entry;

    fn index(&self, index: usize) -> &Entry {
        &self.entries[index]
    }
}

impl<L> IndexMut<usize> for Table<L>
    where L: TableLevel
{
    fn index_mut(&mut self, index: usize) -> &mut Entry {
        &mut self.entries[index]
    }
}


pub trait TableLevel {}

#[cfg(target_arch = "x86_64")] pub enum Level4 {}
#[cfg(target_arch = "x86_64")] pub enum Level3 {}
pub enum Level2 {}
pub enum Level1 {}

#[cfg(target_arch = "x86_64")] impl TableLevel for Level4 {}
#[cfg(target_arch = "x86_64")] impl TableLevel for Level3 {}
impl TableLevel for Level2 {}
impl TableLevel for Level1 {}

pub trait HierarchicalLevel: TableLevel {
    type NextLevel: TableLevel;
}

#[cfg(target_arch = "x86_64")]
impl HierarchicalLevel for Level4 {
    type NextLevel = Level3;
}

#[cfg(target_arch = "x86_64")]
impl HierarchicalLevel for Level3 {
    type NextLevel = Level2;
}

impl HierarchicalLevel for Level2 {
    type NextLevel = Level1;
}
