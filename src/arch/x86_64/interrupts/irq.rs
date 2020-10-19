use x86_64::structures::idt::InterruptStackFrame;

use crate::device::{pic_8259 as pic, serial::uart_16550 as serial};

pub extern "x86-interrupt" fn timer(_stack_frame: &mut InterruptStackFrame) {
    pic::MAIN.lock().ack();
    // TODO: Pre-empt tasks
}

pub extern "x86-interrupt" fn keyboard(_stack_frame: &mut InterruptStackFrame) {
    use crate::device::ps2_controller_8042;

    // Read a single scancode off our keyboard port.
    let code = ps2_controller_8042::key_read();
    crate::device::keyboard::add_scancode(code);

    pic::MAIN.lock().ack();
}

#[allow(unused_variables)]
pub extern "x86-interrupt" fn cascade(_stack_frame: &mut InterruptStackFrame) {
    pic::MAIN.lock().ack();
}

pub extern "x86-interrupt" fn com1(_stack_frame: &mut InterruptStackFrame) {
    let mut com1 = serial::COM1.lock();
    while com1.line_sts().contains(serial::LineStsFlags::DATA_READY) {
        crate::device::serial::add_byte(com1.receive());
    }
    pic::MAIN.lock().ack();
}

pub extern "x86-interrupt" fn com2(_stack_frame: &mut InterruptStackFrame) {
    let mut com2 = serial::COM2.lock();
    while com2.line_sts().contains(serial::LineStsFlags::DATA_READY) {
        crate::device::serial::add_byte(com2.receive());
    }
    pic::MAIN.lock().ack();
}
