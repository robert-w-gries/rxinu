use os_bootinfo::BootInfo;
use x86_64::structures::paging::{PageTable, RecursivePageTable};

#[macro_use]
pub mod console;
pub mod context;
mod device;
mod gdt;
mod idt;
pub mod interrupts;
pub mod memory;

pub fn init(boot_info_address: usize) {
    let boot_info: &BootInfo = unsafe { &*(boot_info_address as *mut BootInfo) };

    if boot_info.check_version().is_err() {
        panic!("os_bootinfo version passed by bootloader does not match crate version!");
    }

    for region in boot_info.memory_map.iter() {
        kprintln!("{:?}", region);
    }

    let mut page_table: &mut PageTable =
        unsafe { &mut *(boot_info.p4_table_addr as *mut PageTable) };

    let rec_page_table =
        RecursivePageTable::new(&mut page_table).expect("recursive page table creation failed");

    let _memory_controller = memory::init(boot_info, rec_page_table);

    unsafe {
        use self::memory::heap::{HEAP_SIZE, HEAP_START};
        ::HEAP_ALLOCATOR.init(HEAP_START as usize, HEAP_SIZE as usize);
    }

    idt::init();
    device::init();
}
