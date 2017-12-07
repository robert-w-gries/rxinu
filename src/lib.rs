#![feature(abi_x86_interrupt)]
#![feature(alloc, allocator_api, global_allocator)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(const_max_value)]
#![feature(const_unique_new, const_atomic_usize_new)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(lang_items)]
#![feature(naked_functions)]
#![feature(unique)]
#![no_std]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate once;

extern crate bit_field;
extern crate compiler_builtins;
extern crate linked_list_allocator;
extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate volatile;
extern crate x86;

#[macro_use]
pub mod arch;
pub mod device;
pub mod scheduling;
pub mod syscall;

#[no_mangle]
/// Entry point for rust code
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    arch::init(multiboot_information_address);
    kprintln!("\nIt did not crash!");

    unsafe {
        HEAP_ALLOCATOR.lock().init(HEAP_START, HEAP_START + HEAP_SIZE);
    }

    use scheduling::{DoesScheduling, Process, Scheduler};

    let mut scheduler = Scheduler::new();

    let main_proc: Process = scheduler.create(rxinu_main).expect("Could not create process!");
    scheduler.ready(main_proc.pid);

    // TODO: Ready multiple processes at once
    //let main_proc2: Process = scheduler.create(rxinu_main).expect("Could not create process!");
    //scheduler.ready(main_proc2.pid);

    // TODO: Investigate returning from null process
    loop {
        unsafe { scheduler.resched(); }
    }
}

/// Main initialization process for rxinu
pub extern fn rxinu_main() {
    arch::console::clear_screen();

    kprintln!("In main process!");
}

#[cfg(not(test))]
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[cfg(not(test))]
#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &str, line: u32) -> ! {
    kprintln!("\n\nPANIC in {} at line {}:", file, line);
    kprintln!("    {}", fmt);
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

use linked_list_allocator::LockedHeap;

const HEAP_START: usize = 0o_000_001_000_000_0000;
const HEAP_SIZE: usize = 100 * 1024;

#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();
