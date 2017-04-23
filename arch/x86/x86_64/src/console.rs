use core::fmt::{self, Write};
use spin::Mutex;

use device;

pub static CONSOLE: Mutex<Console> = Mutex::new(Console);

pub struct Console;

impl Write for Console {
    #[cfg(feature = "vga")]
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        device::vga::VGA.lock().write_str(s)
    }

    #[cfg(feature = "serial")]
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        device::serial::COM1.lock().write_str(s)
    }
}

pub fn init() {
    #[cfg(feature = "vga")]
    device::vga::VGA.lock().clear_screen();

    #[cfg(feature = "serial")]
    device::serial::COM1.lock().clear_screen();
}
