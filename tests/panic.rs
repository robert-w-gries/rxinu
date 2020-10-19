#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rxinu::test::{exit_qemu, QemuExitCode};
use rxinu::{serial_print, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_panic();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

fn should_panic() {
    serial_print!("panic::should_panic...\t");
    panic!();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
