#![no_std]
#![no_main]
#![feature(async_closure)]
#![feature(custom_test_frameworks)]
#![test_runner(rxinu::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use core::sync::atomic::{AtomicBool, Ordering};
use rxinu::task::{CooperativeExecutor, Scheduler, Task};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    rxinu::arch::init(boot_info);
    test_main();
    loop {}
}

#[test_case]
fn create_and_run() {
    let has_run = AtomicBool::new(false);
    let test_process = async move || {
        has_run.store(true, Ordering::SeqCst);
        assert!(has_run.load(Ordering::SeqCst));
    };
    let mut executor = CooperativeExecutor::new();
    executor.spawn(Task::new(test_process())).unwrap();
    executor.run_ready_tasks();
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

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    rxinu::test::test_panic_handler(info);
}
