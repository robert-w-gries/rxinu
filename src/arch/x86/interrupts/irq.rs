use arch::x86::interrupts;
use arch::x86::interrupts::exception::ExceptionStack;
use device::pic_8259 as pic;
use device::uart_16550 as serial;
use device::keyboard::ps2::PS2_KEYBOARD;
use device::pit::PIT_TICKS;
use core::sync::atomic::Ordering;
use scheduling::{DoesScheduling, SCHEDULER};

pub extern "x86-interrupt" fn timer(_stack_frame: &mut ExceptionStack) {
    //This counter variable is updated every time an timer interrupt occurs. The timer is set to
    //interrupt every 2ms, so this means a reschedule will occur if 20ms have passed.
    if PIT_TICKS.fetch_add(1, Ordering::SeqCst) >= 10 {
        PIT_TICKS.store(0, Ordering::SeqCst);

        // ensure that timer is renabled before entering new process
        pic::MASTER.lock().ack();

        //Find another process to run.
        unsafe {
            interrupts::disable_then_restore(|| {
                SCHEDULER.resched();
            })
        };
    } else {
        pic::MASTER.lock().ack();
    }
}

pub extern "x86-interrupt" fn keyboard(_stack_frame: &mut ExceptionStack) {
    interrupts::disable_then_restore(|| {
        use device::BufferedDevice;
        use device::ps2_controller_8042;

        pic::MASTER.lock().ack();

        // Read a single scancode off our keyboard port.
        let code = ps2_controller_8042::key_read();

        // Pass scan code to ps2 driver buffer
        PS2_KEYBOARD.lock().buffer_mut().push_back(code);
    });
}

#[allow(unused_variables)]
pub extern "x86-interrupt" fn cascade(_stack_frame: &mut ExceptionStack) {
    interrupts::disable_then_restore(|| {
        pic::MASTER.lock().ack();
    });
}

pub extern "x86-interrupt" fn com1(_stack_frame: &mut ExceptionStack) {
    interrupts::disable_then_restore(|| {
        pic::MASTER.lock().ack();
        serial::COM1.lock().receive();
    });
}

pub extern "x86-interrupt" fn com2(_stack_frame: &mut ExceptionStack) {
    interrupts::disable_then_restore(|| {
        pic::MASTER.lock().ack();
        serial::COM2.lock().receive();
    });
}
