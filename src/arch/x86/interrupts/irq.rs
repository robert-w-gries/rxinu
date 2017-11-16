use arch::x86::device::pic;
use arch::x86::device::serial::{COM1, COM2};
use arch::x86::interrupts::idt::ExceptionStackFrame;

#[allow(unused_variables)]
pub extern "x86-interrupt" fn cascade(stack_frame: &mut ExceptionStackFrame) {
    pic::MASTER.lock().ack();
}

pub extern "x86-interrupt" fn com1(stack_frame: &mut ExceptionStackFrame) {
    COM1.lock().receive();
    pic::MASTER.lock().ack();
}

pub extern "x86-interrupt" fn com2(stack_frame: &mut ExceptionStackFrame) {
    COM2.lock().receive();
    pic::MASTER.lock().ack();
}
