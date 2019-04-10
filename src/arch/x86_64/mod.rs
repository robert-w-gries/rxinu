use bootloader::bootinfo::BootInfo;

use crate::HEAP_ALLOCATOR;

pub mod context;
mod device;
pub mod idt;
pub mod interrupts;
pub mod memory;

pub unsafe fn init(boot_info: &'static BootInfo) {
    for region in boot_info.memory_map.iter() {
        kprintln!("{:?}", region);
    }

    memory::init(boot_info);

    use self::memory::heap::{HEAP_SIZE, HEAP_START};
    HEAP_ALLOCATOR.init(HEAP_START as usize, HEAP_SIZE as usize);

    idt::init();
    device::init();
}
