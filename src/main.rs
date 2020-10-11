#![no_std]
#![no_main]
#![feature(async_closure)]
#![feature(custom_test_frameworks)]
#![test_runner(rxinu::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{bootinfo::BootInfo, entry_point};
use core::panic::PanicInfo;
use rxinu::{arch, device, kprintln, task::{CooperativeExecutor, Scheduler, Task}};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    arch::init(boot_info);
    arch::interrupts::clear_mask();

    let mut executor = CooperativeExecutor::new();
    let _ = executor.spawn(Task::new(device::keyboard::print_keypresses()));
    let _ = executor.spawn(Task::new(device::serial::print_serial()));
    executor.run();
}

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    kprintln!("{}", info);

    loop {
        unsafe {
            arch::interrupts::halt();
        }
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rxinu::test::test_panic_handler(info)
}
