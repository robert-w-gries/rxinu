#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rxinu::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::string::String;
use bootloader::{bootinfo::BootInfo, entry_point};
use core::panic::PanicInfo;
use rxinu::{arch, device, syscall, task};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    arch::init(boot_info);

    unsafe {
        task::scheduler::init();
        arch::interrupts::clear_mask();
    }

    #[cfg(test)]
    test_main();

    let _ = syscall::create(String::from("rxinu_main"), 0, rxinu::rxinu_main);

    loop {
        #[cfg(feature = "serial")]
        {
            use device::uart_16550 as uart;
            uart::read(1024);
        }

        #[cfg(feature = "vga")]
        {
            use device::keyboard::ps2 as kbd;
            kbd::read(1024);
        }

        // Save cycles by pausing until next interrupt
        arch::interrupts::pause();
    }
}

#[cfg(not(test))]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    use rxinu::kprintln;
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