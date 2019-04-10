#![feature(alloc)]
#![feature(const_fn)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate rxinu;

#[macro_use]
extern crate lazy_static;

extern crate alloc;
extern crate spin;

use bootloader::bootinfo::BootInfo;
use core::panic::PanicInfo;
use rxinu::exit_qemu;
use rxinu::sync::{IrqLock, Semaphore};
use rxinu::syscall;
use rxinu::task::scheduler::global_sched;
use rxinu::task::{ProcessId, Scheduling, State};

lazy_static! {
    static ref COUNT: IrqLock<Semaphore> = IrqLock::new(Semaphore::new(0));
    static ref READY: IrqLock<Semaphore> = IrqLock::new(Semaphore::new(0));
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    unsafe {
        rxinu::arch::init(boot_info);
        rxinu::task::scheduler::init();
        rxinu::arch::interrupts::clear_mask();
    }

    let p1 = syscall::create(alloc::string::String::from("process1"), 100, loop_process).unwrap();

    let check_state = |p: ProcessId, s: State| global_sched().get_process(p).unwrap().state() == s;

    // Run Process1 - will wait until we signal that we are ready
    let _ = syscall::yield_cpu().unwrap();

    // Ensure that Process1 is waiting then acquire current count
    assert!(check_state(p1, State::Wait));
    assert_eq!(COUNT.lock().count(), 0);

    // Signal that we are ready and increment count in Process1
    let _ = READY.lock().signal().unwrap();

    // Process1 should have ran, incremented COUNT, then waited for READY again
    assert!(check_state(p1, State::Wait));
    assert_eq!(COUNT.lock().count(), 1);

    serial_println!("ok");
    unsafe {
        exit_qemu();
    }

    loop {}
}

pub extern "C" fn loop_process() {
    loop {
        let _ = READY.lock().wait().unwrap();
        let _ = COUNT.lock().signal().unwrap();
    }
}

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
