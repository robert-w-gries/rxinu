use core::fmt::{self, Write};
use spin::Mutex;

#[cfg(feature = "serial")]
use super::device::serial::COM1 as console;

#[cfg(feature = "vga")]
use super::device::vga::VGA as console;

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ({
            use core::fmt::Write;
            let _ = write!($crate::arch::x86::console::CONSOLE.lock(), $($arg)*);
    });
}

#[macro_export]
macro_rules! kprintln {
    ($fmt:expr) => (kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (kprint!(concat!($fmt, "\n"), $($arg)*));
}

pub static CONSOLE: Mutex<Console> = Mutex::new(Console);

pub struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        console.lock().write_str(s)
    }
}

pub fn clear_screen() {
    console.lock().clear_screen();
}
