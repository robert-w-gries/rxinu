#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rxinu::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
extern crate lazy_static;

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rxinu::sync::{IrqLock, Semaphore};
use rxinu::syscall;
use rxinu::task::scheduler::global_sched;
use rxinu::task::{ProcessId, Scheduling, State};

lazy_static! {
    static ref COUNT: IrqLock<Semaphore> = IrqLock::new(Semaphore::new(0));
    static ref READY: IrqLock<Semaphore> = IrqLock::new(Semaphore::new(0));
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    rxinu::arch::init(boot_info);
    unsafe {
        rxinu::task::scheduler::init();
        rxinu::arch::interrupts::clear_mask();
    }

    test_main();
    loop {}
}

#[test_case]
fn ipc() {
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
}

pub extern "C" fn loop_process() {
    loop {
        let _ = READY.lock().wait().unwrap();
        let _ = COUNT.lock().signal().unwrap();
    }
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    rxinu::test::test_panic_handler(info);
}
