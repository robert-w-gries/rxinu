use arch::x86::device::pic;
use arch::x86::device::serial::{COM1, COM2};
use arch::x86::interrupts::idt::ExceptionStackFrame;
use devices::{ps2_controller_8042, ps2_keyboard};

#[allow(dead_code)]
fn trigger(irq: u8) {
    if irq >= 8 {
        pic::SLAVE.lock().mask_set(irq - 8);
        pic::SLAVE.lock().ack();
    } else {
        pic::MASTER.lock().mask_set(irq);
        pic::MASTER.lock().ack();
    }
}

pub extern "x86-interrupt" fn timer(_stack_frame: &mut ExceptionStackFrame) {
    pic::MASTER.lock().ack();
}

pub extern "x86-interrupt" fn keyboard(_stack_frame: &mut ExceptionStackFrame) {
    // Read a single scancode off our keyboard port.
    let code = ps2_controller_8042::key_read();

    // Pass scan code to ps2_keyboard driver
    ps2_keyboard::parse_key(code);

    pic::MASTER.lock().ack();
}

#[allow(unused_variables)]
pub extern "x86-interrupt" fn cascade(_stack_frame: &mut ExceptionStackFrame) {
    pic::MASTER.lock().ack();
}

pub extern "x86-interrupt" fn com1(_stack_frame: &mut ExceptionStackFrame) {
    pic::MASTER.lock().ack();
    let data: u8 = COM1.lock().receive();
    kprint!("{}", data as char);
}

pub extern "x86-interrupt" fn com2(_stack_frame: &mut ExceptionStackFrame) {
    pic::MASTER.lock().ack();
    let data: u8 = COM2.lock().receive();
    kprint!("{}", data as char);
}
