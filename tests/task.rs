#![no_std]
#![no_main]
#![feature(async_closure)]
#![feature(custom_test_frameworks)]
#![test_runner(rxinu::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::sync::Arc;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicBool, Ordering};
use rxinu::task::{self, CooperativeExecutor, Scheduler, Task};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    rxinu::arch::init(boot_info);
    test_main();
    loop {}
}

#[test_case]
fn create_and_run() {
    let has_run1 = Arc::new(AtomicBool::new(false));
    let has_run2 = has_run1.clone();
    let test_process = async move || {
        has_run1.store(true, Ordering::SeqCst);
    };
    let mut executor = CooperativeExecutor::new();
    executor.spawn(Task::new(test_process())).unwrap();
    executor.run_ready_tasks();
    assert!(has_run2.load(Ordering::SeqCst));
}

#[test_case]
fn kill() {
    let kill_process = async move || {
        panic!("Process should have been killed");
    };
    let mut executor = CooperativeExecutor::new();
    let task = Task::new(kill_process());
    let pid = task.id();
    executor.spawn(task).unwrap();
    executor.kill(pid).unwrap();
    executor.run_ready_tasks();
}

/// Spawn Task1 then spawn Task2
/// Task1 yields
/// Task2 sets has_run to true and finishes
/// Task1 returns to yield point and asserts has_run is true
#[test_case]
fn yield_now() {
    let has_run1 = Arc::new(AtomicBool::new(false));
    let has_run2 = has_run1.clone();
    let task1 = async move || {
        task::yield_now().await;
        assert!(has_run1.load(Ordering::SeqCst));
    };
    let task2 = async move || {
        has_run2.store(true, Ordering::SeqCst);
    };
    let mut executor = CooperativeExecutor::new();
    executor.spawn(Task::new(task1())).unwrap();
    executor.spawn(Task::new(task2())).unwrap();
    executor.run_ready_tasks();
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    rxinu::test::test_panic_handler(info);
}
