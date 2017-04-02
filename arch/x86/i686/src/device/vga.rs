use core::ptr::Unique;
use spin::Mutex;
use volatile::Volatile;

const MAX_HEIGHT: usize = 25;
const MAX_WIDTH: usize = 80;
pub const VGA_ADDR: usize = 0xb8000;

pub static VGA: Mutex<Writer> = Mutex::new(Writer {
    column_position: 0,
    color_code: ColorCode::new(Color::LightGreen, Color::Black),
    buffer: unsafe { Unique::new(VGA_ADDR as *mut _) },
});

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>,
}

impl Writer {
	pub fn clear_screen(&mut self) {
	    for row in 0..MAX_HEIGHT {
			self.clear_row(row);
	    }
	}

    fn write_byte(&mut self, byte: &u8) {
        match *byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= MAX_WIDTH {
                    self.new_line();
                }

                let row = MAX_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer().chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.get_mut() }
    }

    fn new_line(&mut self) {
        for row in 1..MAX_HEIGHT {
            for col in 0..MAX_WIDTH {
                let buffer = self.buffer();
                let character = buffer.chars[row][col].read();
                buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(MAX_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for col in 0..MAX_WIDTH {
            self.buffer().chars[row][col].write(blank);
        }
    }
}

impl ::core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.as_bytes() {
            self.write_byte(byte)
        }

        Ok(())
    }
}

struct Buffer {
    pub chars: [[Volatile<ScreenChar>; MAX_WIDTH]; MAX_HEIGHT],
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}
