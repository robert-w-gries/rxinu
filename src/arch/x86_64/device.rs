use crate::device::{pic_8259, pit, ps2_controller_8042, serial::uart_16550};

pub fn init() {
    pic_8259::init();
    unsafe {
        uart_16550::init();
        ps2_controller_8042::init();
    }
    pit::init();
}
