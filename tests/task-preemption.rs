#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rxinu::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicUsize, Ordering};
use rxinu::syscall;

static EXECUTED: AtomicUsize = AtomicUsize::new(0);

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
fn preemption() {
    // Scheduler task selection => process1, process2, NULL_PROCESS
    let _ = syscall::create(alloc::string::String::from("process1"), 11, loop_process).unwrap();
    let _ = syscall::create(alloc::string::String::from("process2"), 10, loop_process).unwrap();

    let _ = syscall::yield_cpu().unwrap();

    // Both processes should run
    assert_eq!(EXECUTED.load(Ordering::SeqCst), 2);
}

pub extern "C" fn loop_process() {
    EXECUTED.fetch_add(1, Ordering::SeqCst);
    loop {}
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    rxinu::test::test_panic_handler(info);
}
