use core::ptr::Unique;
use spin::Mutex;

use self::color::{Color, ColorCode};
use self::writer::Writer;

mod buffer;
mod color;
mod writer;

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::LightGreen, Color::Black),
    buffer: unsafe { Unique::new(0xb8000 as *mut _) },
});

macro_rules! print {
    ($($arg:tt)*) => ({
            use core::fmt::Write;
            let mut writer = $crate::vga_buffer::WRITER.lock();
            writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

pub fn clear_screen() {
    for _ in 0..buffer::MAX_HEIGHT {
        println!("");
    }
}
