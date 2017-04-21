use core::fmt::{self, Write};
use spin::Mutex;

use device::serial::COM1;
use device::vga::VGA;

pub static CONSOLE: Mutex<Console> = Mutex::new(Console);

pub struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        //VGA.lock().write_str(s)
        COM1.lock().write_str(s)
    }
}

pub fn init() {
    VGA.lock().clear_screen();
}
