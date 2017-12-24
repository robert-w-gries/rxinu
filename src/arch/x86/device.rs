use device::{pic_8259, ps2_controller_8042, uart_16550, pit};

pub fn init() {
    pic_8259::init();
    uart_16550::init();
    ps2_controller_8042::init();
    pit::init();
}
