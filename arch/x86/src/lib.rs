#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(unique)]
#![feature(const_unique_new)]
#![no_std]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate once;

extern crate bit_field;
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

// declare modules after exporting print macros
pub mod console;
pub mod device;
pub mod interrupts;
pub mod memory;

#[allow(dead_code)]
fn enable_nxe_bit() {
    use x86::shared::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86::shared::control_regs::*;

    unsafe { cr0_write(cr0() | CR0_WRITE_PROTECT) };
}
