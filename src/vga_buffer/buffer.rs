use vga_buffer::color::ColorCode;
use volatile::Volatile;

pub const MAX_HEIGHT: usize = 25;
pub const MAX_WIDTH: usize = 80;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}

pub struct Buffer {
    pub chars: [[Volatile<ScreenChar>; MAX_WIDTH]; MAX_HEIGHT],
}
