#![feature(alloc)]
#![feature(allocator_api)]
#![feature(global_allocator)]
#![feature(const_fn)]

#![no_std]
#![deny(warnings)]

extern crate alloc;
extern crate spin;
extern crate linked_list_allocator;

use alloc::heap::{Alloc, AllocErr, Layout};
use spin::Mutex;
use linked_list_allocator::Heap;

pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

static HEAP: Mutex<Option<Heap>> = Mutex::new(None);

pub unsafe fn init(offset: usize, size: usize) {
    *HEAP.lock() = Some(Heap::new(offset, size));
}

pub struct Allocator;

unsafe impl<'a> Alloc for &'a Allocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        if let Some(ref mut heap) = *HEAP.lock() {
            heap.allocate_first_fit(layout)
        } else {
            panic!("Heap not initialized!");
        }
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        if let Some(ref mut heap) = *HEAP.lock() {
            heap.deallocate(ptr, layout)
        } else {
            panic!("heap not initalized");
        }
    }

    fn oom(&mut self, _: AllocErr) -> ! {
        panic!("Out of memory");
    }
}

#[global_allocator]
static GLOBAL_ALLOC: Allocator = Allocator;
