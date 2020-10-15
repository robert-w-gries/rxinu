#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rxinu::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{bootinfo::BootInfo, entry_point};
use core::panic::PanicInfo;
use rxinu::{
    arch, device,
    task::{Priority, PriorityTask},
    task::scheduler::{PriorityScheduler, Scheduler},
};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    arch::init(boot_info);
    arch::interrupts::clear_mask();

    #[cfg(test)]
    rxinu::test::exit_qemu(rxinu::test::QemuExitCode::Success);

    let mut executor = PriorityScheduler::new();
    let keyboard_task = PriorityTask::new(Priority::High, device::keyboard::print_keypresses());
    let serial_task = PriorityTask::new(Priority::High, device::serial::print_serial());
    executor.spawn(keyboard_task).unwrap();
    executor.spawn(serial_task).unwrap();
    executor.run();
}

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    rxinu::kprintln!("{}", info);

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
