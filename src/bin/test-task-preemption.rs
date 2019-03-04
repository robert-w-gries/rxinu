#![feature(alloc)]
#![feature(const_fn)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate rxinu;

extern crate alloc;
extern crate spin;

use bootloader::bootinfo::BootInfo;
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicUsize, Ordering};
use rxinu::exit_qemu;

static EXECUTED: AtomicUsize = AtomicUsize::new(0);

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    use rxinu::syscall;

    unsafe {
        rxinu::arch::init(boot_info);
        rxinu::task::scheduler::init();
        rxinu::arch::interrupts::clear_mask();
    }

    // Scheduler task selection => process1, process2, NULL_PROCESS
    let _ = syscall::create(alloc::string::String::from("process1"), 11, loop_process).unwrap();
    let _ = syscall::create(alloc::string::String::from("process2"), 10, loop_process).unwrap();

    let _ = syscall::yield_cpu().unwrap();

    // Both processes should run
    assert_eq!(EXECUTED.load(Ordering::SeqCst), 2);

    serial_println!("ok");
    unsafe {
        exit_qemu();
    }

    loop {}
}

pub extern "C" fn loop_process() {
    EXECUTED.fetch_add(1, Ordering::SeqCst);
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    unsafe {
        rxinu::arch::interrupts::disable();
    }

    serial_println!("failed");
    serial_println!("{}", info);

    unsafe {
        exit_qemu();
    }

    loop {}
}
