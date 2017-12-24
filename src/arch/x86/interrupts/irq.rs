use arch::x86::interrupts::exception::ExceptionStack;
use device::{ps2_controller_8042, ps2_keyboard};
use device::pic_8259 as pic;
use device::uart_16550 as serial;
use device::pit::PIT_TICKS;
use core::sync::atomic::Ordering;
use scheduling::{SCHEDULER, DoesScheduling};

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

pub extern "x86-interrupt" fn timer(_stack_frame: &mut ExceptionStack) {
    pic::MASTER.lock().ack();

    if PIT_TICKS.fetch_add(1, Ordering::SeqCst) >= 10 {
        unsafe {
            SCHEDULER.resched();
        } 
    }
}

pub extern "x86-interrupt" fn keyboard(_stack_frame: &mut ExceptionStack) {
    // Read a single scancode off our keyboard port.
    let code = ps2_controller_8042::key_read();

    // Pass scan code to ps2_keyboard driver
    ps2_keyboard::parse_key(code);

    pic::MASTER.lock().ack();
}

#[allow(unused_variables)]
pub extern "x86-interrupt" fn cascade(_stack_frame: &mut ExceptionStack) {
    pic::MASTER.lock().ack();
}

pub extern "x86-interrupt" fn com1(_stack_frame: &mut ExceptionStack) {
    pic::MASTER.lock().ack();
    let data: u8 = serial::COM1.lock().receive();
    kprint!("{}", data as char);
}

pub extern "x86-interrupt" fn com2(_stack_frame: &mut ExceptionStack) {
    pic::MASTER.lock().ack();
    let data: u8 = serial::COM2.lock().receive();
    kprint!("{}", data as char);
}
