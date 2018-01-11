#[macro_use]
pub mod keyboard;

pub mod pic_8259;
pub mod pit;
pub mod ps2_controller_8042;
pub mod uart_16550;
pub mod vga;

use alloc::VecDeque;

pub trait BufferedDevice {
    fn buffer(&self) -> &VecDeque<u8>;
    fn buffer_mut(&mut self) -> &mut VecDeque<u8>;
}

pub trait InputDevice {
    fn read(&mut self, num_bytes: usize) -> VecDeque<char>;
}

pub trait OutputDevice {
    fn write(&mut self, data: VecDeque<char>);
}
