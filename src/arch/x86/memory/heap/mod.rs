/// Use allocator wrapper, similar to what le-jzr/sisyphos-kernel-uefi-x86_64 uses

pub mod bump_allocator;

use alloc::allocator::{Alloc, AllocErr, Layout};
use arch::interrupts;
use linked_list_allocator::LockedHeap;

pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 500 * 1024; // 500 KB

pub struct HeapAllocator {
    inner: LockedHeap,
}

impl HeapAllocator {
    /// Creates an empty heap. All allocate calls will return `None`.
    pub const fn new() -> Self {
        HeapAllocator { inner: LockedHeap::empty() }
    }

    /// Initializes an empty heap
    ///
    /// # Unsafety
    ///
    /// This function must be called at most once and must only be used on an
    /// empty heap.  Also, it is assumed that interrupts are disabled.
    pub unsafe fn init(&self, heap_bottom: usize, heap_size: usize) {
        self.inner.lock().init(heap_bottom, heap_size);
    }
}

/// Wrappers for inner Alloc implementation
unsafe impl<'a> Alloc for &'a HeapAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        interrupts::disable_then_restore(|| -> Result<*mut u8, AllocErr> {
            self.inner.lock().alloc(layout)
        })
    }

    #[inline]
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        interrupts::disable_then_restore(|| {
            self.inner.lock().dealloc(ptr, layout);
        });
    }

    fn oom(&mut self, _: AllocErr) -> ! {
        panic!("Out of memory");
    }
}
