use core::fmt::{self, Write};
use spin::Mutex;

#[cfg(feature = "serial")]
use device::serial::COM1 as console;

#[cfg(feature = "vga")]
use device::vga::VGA as console;

pub static CONSOLE: Mutex<Console> = Mutex::new(Console);

pub struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        console.lock().write_str(s)
    }
}

pub fn init() {
    console.lock().clear_screen();
}
