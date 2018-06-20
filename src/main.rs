#![feature(alloc, lang_items, panic_info_message)]
#![no_std]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate rxinu;
extern crate alloc;

use alloc::String;
use core::panic::PanicInfo;
use rxinu::{arch, device, syscall, task};

#[no_mangle]
/// Entry point for rust code
pub extern "C" fn _start(boot_info_address: usize) -> ! {
    use arch::memory::heap::{HEAP_SIZE, HEAP_START};

    arch::init(boot_info_address);

    unsafe {
        task::scheduler::init();
        arch::interrupts::clear_mask();
    }

    kprintln!("\nHEAP START = 0x{:x}", HEAP_START);
    kprintln!("HEAP END = 0x{:x}\n", HEAP_START + HEAP_SIZE);

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
#[lang = "panic_impl"]
#[no_mangle]
pub extern "C" fn panic_fmt(info: &PanicInfo) -> ! {
    arch::interrupts::disable_then_execute(|| {
        kprintln!("\n\nPANIC");

        if let Some(location) = info.location() {
            kprint!("in {} at line {}", location.file(), location.line());
        }

        if let Some(message) = info.message() {
            kprintln!("\n    {:?}", message);
        }
    });

    loop {
        unsafe {
            arch::interrupts::halt();
        }
    }
}
