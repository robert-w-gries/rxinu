use devices::ps2_keyboard;

pub mod pic;
pub mod serial;
pub mod vga;

pub fn init() {
    pic::init();
    serial::init();
    ps2_keyboard::init();
}
