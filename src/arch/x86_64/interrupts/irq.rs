use x86_64::structures::idt::ExceptionStackFrame;

use crate::device::{keyboard::ps2::PS2_KEYBOARD, pic_8259 as pic, uart_16550 as serial};
use crate::task::scheduler::{global_sched, Scheduling};

pub extern "x86-interrupt" fn timer(_stack_frame: &mut ExceptionStackFrame) {
    pic::MASTER.lock().ack();
    global_sched().tick();
}

pub extern "x86-interrupt" fn keyboard(_stack_frame: &mut ExceptionStackFrame) {
    use crate::device::{ps2_controller_8042, BufferedDevice};

    // Read a single scancode off our keyboard port.
    let code = ps2_controller_8042::key_read();

    // Pass scan code to ps2 driver buffer
    PS2_KEYBOARD.lock().buffer_mut().push_back(code);

    pic::MASTER.lock().ack();
}

#[allow(unused_variables)]
pub extern "x86-interrupt" fn cascade(_stack_frame: &mut ExceptionStackFrame) {
    pic::MASTER.lock().ack();
}

pub extern "x86-interrupt" fn com1(_stack_frame: &mut ExceptionStackFrame) {
    serial::COM1.lock().receive();
    pic::MASTER.lock().ack();
}

pub extern "x86-interrupt" fn com2(_stack_frame: &mut ExceptionStackFrame) {
    serial::COM2.lock().receive();
    pic::MASTER.lock().ack();
}
