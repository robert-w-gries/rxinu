#![feature(alloc, panic_implementation)]
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

use core::panic::PanicInfo;
use rxinu::exit_qemu;
use rxinu::sync::{IrqLock, Semaphore};
use rxinu::syscall;
use rxinu::task::{ProcessId, Scheduling, State};
use rxinu::task::scheduler::global_sched;

lazy_static! {
    static ref SEM: IrqLock<Semaphore> = IrqLock::new(Semaphore::new(2));
    static ref COUNT: IrqLock<Semaphore> = IrqLock::new(Semaphore::new(0));
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start(boot_info_address: usize) -> ! {
    unsafe {
        rxinu::arch::init(boot_info_address);
        rxinu::task::scheduler::init();
        rxinu::arch::interrupts::clear_mask();
    }

    let p1 = syscall::create(alloc::String::from("process1"), 100, loop_process).unwrap();

    let check_state = |p: ProcessId, s: State| global_sched().get_process(p).unwrap().state() == s;

    // Run Process1
    let _ = syscall::yield_cpu().unwrap();

    assert!(check_state(p1, State::Wait));
    let sem_count = COUNT.lock().count();

    // Process1 runs again then enters WAIT state again
    SEM.lock().signal().unwrap();

    assert!(check_state(p1, State::Wait));
    assert!(sem_count < COUNT.lock().count());

    serial_println!("ok");
    unsafe {
        exit_qemu();
    }

    loop{}
}

pub extern "C" fn loop_process() {
    loop {
        let _ = COUNT.lock().signal().unwrap();
        let _ = SEM.lock().wait().unwrap();
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
