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

/* TODO: Implement usermode
/// Enter usermode.
/// To enter Ring3, we must pretend to raise an inter-privilege level interrupt.
/// [unsafe]
/// This function is pure assembly and is inherently unsafe
#[allow(unreachable_code)]
pub unsafe fn enter_usermode(ip: usize, sp: usize) -> ! {
    use x86::shared::flags::{FLAGS_IOPL0, Flags, FLAGS_IF};

    gdt::load_selectors(gdt::GDT_USER_DATA, PrivilegeLevel::Ring3);

    // Setup stack
    asm!("push r10
        push r11
        push r12
        push r13
        push r14"
        : // No output
        : "{r10}"(USER_DATA.bits()),
          "{r11}"(sp),
          "{r12}"(Flags::new() | FLAGS_IOPL0 | FLAGS_IF), // Enable interrupts
          "{r13}"(USER_CODE.bits()),
          "{r15}"(ip)
        : // Doesn't matter because function does not return
        : "intel", "volatile");

    // execute interrupt return then execute in usermode
    execute_ring3_code();
}

/// Execute interrupt return to enter userspace
unsafe fn execute_ring3_code() -> ! {
    asm!("iretq");
    unreachable!();
}
*/

#[inline]
pub unsafe fn halt() {
    asm!("hlt");
}
