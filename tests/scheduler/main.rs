#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rxinu::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

mod priority;
mod round_robin;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    rxinu::arch::init(boot_info);
    test_main();
    loop {}
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    rxinu::test::test_panic_handler(info);
}
