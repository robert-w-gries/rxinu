#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rxinu::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::string::String;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicBool, Ordering};
use rxinu::serial_println;
use rxinu::syscall;

static SUSPENDED_PROC: AtomicBool = AtomicBool::new(false);

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    rxinu::arch::init(boot_info);

    unsafe {
        rxinu::task::scheduler::init();
    }

    test_main();
    loop {}
}

#[test_case]
fn create() {
    extern "C" fn test_process() {}
    let _ = rxinu::syscall::create(
        alloc::string::String::from("test process!"),
        0,
        test_process,
    )
    .unwrap();
}

#[test_case]
fn create_and_run() {
    // TODO: implement closures as valid tasks
    // let has_run = AtomicBool::new(false);
    extern "C" fn test_process() {
        // TODO
        // has_run.store(true, Ordering::SeqCst);
    }
    assert!(rxinu::syscall::create(
        alloc::string::String::from("test process!"),
        0,
        test_process,
    ).is_ok());

    let _ = rxinu::syscall::yield_cpu().unwrap();

    // TODO
    // assert!(has_run.load(Ordering::SeqCst));
}

#[test_case]
fn kill() {
    extern "C" fn kill_process() {}
    let kill_pid = syscall::create(String::from("kill"), 0, kill_process).unwrap();
    assert!(syscall::kill(kill_pid).is_ok());
}

#[test_case]
fn suspend_and_resume() {
    pub extern "C" fn loop_process() {
        loop {
            let _ = rxinu::syscall::yield_cpu().unwrap();
        }
    }

    pub extern "C" fn suspend_process() {
        assert!(SUSPENDED_PROC.load(Ordering::SeqCst));
        serial_println!("resumed");
    }

    let suspend_pid = syscall::create(String::from("suspend"), 0, suspend_process).unwrap();
    assert!(syscall::suspend(suspend_pid).is_ok());

    SUSPENDED_PROC.store(true, Ordering::SeqCst);

    let _ = syscall::create(String::from("loop"), 0, loop_process).unwrap();
    let _ = syscall::yield_cpu().unwrap();

    assert!(syscall::resume(suspend_pid).is_ok());
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    rxinu::test::test_panic_handler(info);
}