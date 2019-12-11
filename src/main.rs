#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

#[macro_use]
extern crate rxinu;
extern crate alloc;

use bootloader::{bootinfo::BootInfo, entry_point};
use core::panic::PanicInfo;
use rxinu::{
    arch::{self, memory::{HEAP_SIZE, HEAP_START}},
    device,
    task
};

entry_point!(kernel_main);

#[cfg(not(test))]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    unsafe {
        arch::init(boot_info);
        task::scheduler::init();
        arch::interrupts::clear_mask();
    }

    kprintln!("\nHEAP START = 0x{:x}", HEAP_START);
    kprintln!("HEAP END = 0x{:x}\n", HEAP_START + HEAP_SIZE);

    // TODO: Do we care if the rxinu_main process doesn't run?
    let _ = task::Process::new("rxinu_main", 0, rxinu::rxinu_main).spawn();

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
    kprintln!("{}", info);

    loop {
        unsafe {
            arch::interrupts::halt();
        }
    }
}
