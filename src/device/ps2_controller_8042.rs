use crate::syscall::io::{Io, Port};
use spin::Mutex;

/// Our global keyboard state, protected by a mutex.
static CONTROLLER: Mutex<Port<u8>> = Mutex::new(Port::new(0x64));
static DEVICE: Mutex<Port<u8>> = Mutex::new(Port::new(0x60));

pub fn init() {
    // Poll bit 1 of Status Register "Input buffer empty/full"
    let wait_then_write = |data: u8| {
        while CONTROLLER.lock().read() & 0x2 == 1 {}
        DEVICE.lock().write(data);
    };

    // Poll bit 0 of Status Register "Output buffer empty/full"
    let wait_then_read = || -> u8 {
        while CONTROLLER.lock().read() & 0x1 == 0 {}
        DEVICE.lock().read()
    };

    // Disable PS/2 devices
    CONTROLLER.lock().write(0xAD);
    CONTROLLER.lock().write(0xA7);

    // Flush the output buffer
    DEVICE.lock().read();

    // Setup the Controller Configuration Byte
    CONTROLLER.lock().write(0x20);
    let mut config_byte: u8 = wait_then_read();

    // TODO: see if two ports exist

    // Disable all IRQs
    config_byte &= !(1 << 0);
    config_byte &= !(1 << 1);

    // write new configuration
    CONTROLLER.lock().write(0x60);
    wait_then_write(config_byte);

    // Perform Controller Self Test
    CONTROLLER.lock().write(0xAA);
    assert!(
        wait_then_read() == 0x55,
        "PS/2 Controller self check failed!"
    );

    // Perform interface tests
    CONTROLLER.lock().write(0xAB);
    assert!(wait_then_read() == 0x0, "PS/2 Interface test failed!");

    // Enable device(s)
    CONTROLLER.lock().write(0xAE);

    // Get current configuration byte
    CONTROLLER.lock().write(0x20);
    let mut enable: u8 = wait_then_read();

    // Enable all IRQs
    enable |= 1 << 0;

    // write new configuration
    CONTROLLER.lock().write(0x60);
    wait_then_write(enable);

    // Flush buffer
    DEVICE.lock().read();

    kprintln!("[ OK ] PS/2 Driver");
}

pub fn key_read() -> u8 {
    DEVICE.lock().read()
}
