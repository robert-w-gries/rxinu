use devices::ps2_controller_8042;

pub mod pic;
pub mod serial;
pub mod vga;

pub fn init() {
    pic::init();
    serial::init();
    ps2_controller_8042::init();
}
