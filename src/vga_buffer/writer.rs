use core::ptr::Unique;

use vga_buffer::buffer;
use vga_buffer::color::ColorCode;

pub struct Writer {
    pub column_position: usize,
    pub color_code: ColorCode,
    pub buffer: Unique<buffer::Buffer>,
}

impl Writer {
    fn write_byte(&mut self, byte: &u8) {
        match *byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= buffer::MAX_WIDTH {
                    self.new_line();
                }

                let row = buffer::MAX_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer().chars[row][col].write(buffer::ScreenChar {
                    ascii_character: byte,
                    color_code: color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn buffer(&mut self) -> &mut buffer::Buffer {
        unsafe { self.buffer.get_mut() }
    }

    fn new_line(&mut self) {
        for row in 1..buffer::MAX_HEIGHT {
            for col in 0..buffer::MAX_WIDTH {
                let buffer = self.buffer();
                let character = buffer.chars[row][col].read();
                buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(buffer::MAX_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = buffer::ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };

        for col in 0..buffer::MAX_WIDTH {
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
