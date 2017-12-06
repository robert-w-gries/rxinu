#![feature(abi_x86_interrupt)]
#![feature(alloc)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(const_max_value)]
#![feature(const_unique_new)]
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
extern crate hole_list_allocator as allocator;
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

    use scheduling::{DoesScheduling, Process, Scheduler};
    use scheduling::scheduler;

    let scheduler: &'static Scheduler = scheduler();

    let main_proc: Process = scheduler.create(rxinu_main).expect("Could not create process!");
    scheduler.ready(main_proc.pid);
    scheduler.resched();

    kprintln!("Out of main!");

    loop {}
}

// TODO: Fix interrupts re-enabling
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
