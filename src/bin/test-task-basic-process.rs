#![feature(alloc)]
#![feature(const_fn)]
#![no_std] // don't link the Rust standard library
#![cfg_attr(not(test), no_main)] // disable all Rust-level entry points
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

extern crate alloc;

#[macro_use]
extern crate rxinu;

use core::panic::PanicInfo;
use rxinu::exit_qemu;

/// This function is the entry point, since the linker looks for a function
/// named `_start` by default.
#[cfg(not(test))]
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start(boot_info_address: usize) -> ! {
    unsafe {
        rxinu::arch::init(boot_info_address);
        rxinu::task::scheduler::init();
    }

    let _ = rxinu::syscall::create(
        alloc::string::String::from("test process!"),
        0,
        test_process,
    ).unwrap();

    let _ = rxinu::syscall::yield_cpu().unwrap();

    serial_println!("failed");
    serial_println!("Process did not run.");

    unsafe {
        exit_qemu();
    }

    loop {}
}

pub extern "C" fn test_process() {
    serial_println!("ok");
    unsafe {
        exit_qemu();
    }
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    serial_println!("failed");
    serial_println!("{}", info);

    unsafe {
        exit_qemu();
    }
    loop {}
}
