use arch::x86::device::pic;
use arch::x86::device::serial::{COM1, COM2};
use arch::x86::interrupts::idt::ExceptionStackFrame;
use devices::ps2_keyboard;

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
    pic::MASTER.lock().ack();
    if let Some(input) = ps2_keyboard::read_char() {
        match input {
            '\n' => { println!(""); }
            input => { print!("Input = {}\n\n", input); }
        }
    }
}

#[allow(unused_variables)]
pub extern "x86-interrupt" fn cascade(_stack_frame: &mut ExceptionStackFrame) {
    pic::MASTER.lock().ack();
}

pub extern "x86-interrupt" fn com1(_stack_frame: &mut ExceptionStackFrame) {
    COM1.lock().receive();
    pic::MASTER.lock().ack();
}

pub extern "x86-interrupt" fn com2(_stack_frame: &mut ExceptionStackFrame) {
    COM2.lock().receive();
    pic::MASTER.lock().ack();
}
