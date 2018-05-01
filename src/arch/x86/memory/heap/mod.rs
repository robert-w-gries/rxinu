/// Use allocator wrapper, similar to what le-jzr/sisyphos-kernel-uefi-x86_64 uses

pub mod bump_allocator;

use arch::interrupts;
use core::alloc::{Alloc, GlobalAlloc, Layout, Opaque};
use core::ptr::NonNull;
use linked_list_allocator::LockedHeap;

pub const HEAP_START: u64 = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: u64 = 500 * 1024; // 500 KB

pub struct HeapAllocator {
    inner: LockedHeap,
}

impl HeapAllocator {
    /// Creates an empty heap. All allocate calls will return `None`.
    pub const fn new() -> Self {
        HeapAllocator {
            inner: LockedHeap::empty(),
        }
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
unsafe impl GlobalAlloc for HeapAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut Opaque {
        interrupts::disable_then_restore(|| -> *mut Opaque {
            self.inner.lock().alloc(layout).ok().map_or(0 as *mut Opaque, |allocation| allocation.as_ptr())
        })
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut Opaque, layout: Layout) {
        interrupts::disable_then_restore(|| {
            self.inner.lock().dealloc(NonNull::new_unchecked(ptr), layout);
        });
    }
}
