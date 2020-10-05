#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rxinu::serial_println;
use rxinu::test::{QemuExitCode, exit_qemu};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_panic();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn should_panic() {
    panic!();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}