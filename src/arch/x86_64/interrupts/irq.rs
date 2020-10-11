use x86_64::structures::idt::InterruptStackFrame;

use crate::device::{pic_8259 as pic, uart_16550 as serial};

pub extern "x86-interrupt" fn timer(_stack_frame: &mut InterruptStackFrame) {
    pic::MASTER.lock().ack();
    // TODO: Pre-empt tasks
}

pub extern "x86-interrupt" fn keyboard(_stack_frame: &mut InterruptStackFrame) {
    use crate::device::{ps2_controller_8042};

    // Read a single scancode off our keyboard port.
    let code = ps2_controller_8042::key_read();
    crate::device::keyboard::add_scancode(code);

    pic::MASTER.lock().ack();
}

#[allow(unused_variables)]
pub extern "x86-interrupt" fn cascade(_stack_frame: &mut InterruptStackFrame) {
    pic::MASTER.lock().ack();
}

pub extern "x86-interrupt" fn com1(_stack_frame: &mut InterruptStackFrame) {
    serial::COM1.lock().receive();
    pic::MASTER.lock().ack();
}

pub extern "x86-interrupt" fn com2(_stack_frame: &mut InterruptStackFrame) {
    serial::COM2.lock().receive();
    pic::MASTER.lock().ack();
}
