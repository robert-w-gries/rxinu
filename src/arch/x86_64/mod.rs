use bootloader::bootinfo::BootInfo;
use x86_64::VirtAddr;

pub mod context;
mod device;
pub mod idt;
pub mod interrupts;
pub mod memory;

pub fn init(boot_info: &'static BootInfo) {
    for region in boot_info.memory_map.iter() {
        kprintln!("{:?}", region);
    }

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    memory::heap::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    idt::init();
    device::init();
}
