#![feature(const_fn)]
#![feature(unique)]
#![feature(const_unique_new)]
#![no_std]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate once;

extern crate hole_list_allocator as allocator;

extern crate multiboot2;
extern crate rlibc;
extern crate spin;
extern crate volatile;
extern crate x86;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
            use core::fmt::Write;
            let _ = write!($crate::console::CONSOLE.lock(), $($arg)*);
    });
}

#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

fn enable_write_protect_bit() {
    use x86::shared::control_regs::{cr0, cr0_write, CR0_WRITE_PROTECT};

    unsafe { cr0_write(cr0() | CR0_WRITE_PROTECT) };
}

pub mod console;
pub mod device;
pub mod memory;
