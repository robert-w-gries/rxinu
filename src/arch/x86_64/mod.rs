use bootloader::bootinfo::BootInfo;
use x86_64::VirtAddr;

mod device;
pub mod gdt;
mod idt;
pub mod interrupts;
pub mod memory;

pub fn init(boot_info: &'static BootInfo) {
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    memory::heap::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    gdt::init();
    idt::init();
    device::init();
}
