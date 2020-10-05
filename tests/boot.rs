#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rxinu::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rxinu::kprintln;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    rxinu::test::test_panic_handler(info);
}

#[test_case]
fn test_println() {
    kprintln!("test_println output");
}