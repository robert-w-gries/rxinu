#![feature(alloc, panic_implementation)]
#![feature(const_fn)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate rxinu;

extern crate alloc;
extern crate spin;

use rxinu::exit_qemu;
use core::panic::PanicInfo;
use spin::Mutex;

static LOOPED: Mutex<bool> = Mutex::new(false);

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start(boot_info_address: usize) -> ! {
    use rxinu::syscall;

    unsafe {
        rxinu::arch::init(boot_info_address);
        rxinu::task::scheduler::init();
        rxinu::arch::interrupts::clear_mask();
    }

    let _ = syscall::create(alloc::String::from("process1"), 5, loop_process).unwrap();
    let _ = syscall::create(alloc::String::from("process2"), 5, loop_process).unwrap();

    let _ = syscall::yield_cpu().unwrap();

    assert!(*LOOPED.lock());

    serial_println!("ok");
    unsafe {
        exit_qemu();
    }

    loop{}
}

pub extern "C" fn loop_process() {
    *LOOPED.lock() = true;
    loop {
        rxinu::arch::interrupts::pause();
    }
}

#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    serial_println!("failed");

    serial_println!("{}", info);
    unsafe {
        exit_qemu();
    }
    loop {}
}
