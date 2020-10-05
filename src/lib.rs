#![no_std]
#![cfg_attr(test, no_main)]
#![feature(
    abi_x86_interrupt,
    allocator_api,
    alloc_error_handler,
    const_fn,
    const_in_array_repeat_expressions,
    const_mut_refs,
    custom_test_frameworks,
    global_asm,
    lang_items,
    llvm_asm,
    naked_functions,
    ptr_internals
)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
extern crate alloc;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

#[macro_use]
pub mod device;

#[macro_use]
pub mod arch;

pub mod sync;
pub mod syscall;
pub mod task;
pub mod test;

/// Main initialization process for rxinu
pub extern "C" fn rxinu_main() {
    device::console::clear_screen();
    kprintln!("In main process!\n");
    serial_println!("In main process!\n");
}

#[alloc_error_handler]
pub fn rust_oom(info: core::alloc::Layout) -> ! {
    panic!("{:?}", info);
}

#[cfg(test)]
bootloader::entry_point!(test_kernel_main);

#[cfg(test)]
fn test_kernel_main(boot_info: &'static bootloader::bootinfo::BootInfo) -> ! {
    arch::init(boot_info);
    test_main();

    loop {
        unsafe {
            arch::interrupts::halt();
        }
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    test::test_panic_handler(info)
}