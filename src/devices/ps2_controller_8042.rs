use spin::Mutex;
use syscall::io::{Io, Port};

/// Our global keyboard state, protected by a mutex.
static CONTROLLER: Mutex<Port<u8>> = Mutex::new(Port::new(0x64));
static DEVICE: Mutex<Port<u8>> = Mutex::new(Port::new(0x60));

pub fn init() {
    // Disable PS/2 devices
    CONTROLLER.lock().write(0xAD);
    CONTROLLER.lock().write(0xA7);

    // Flush the output buffer
    DEVICE.lock().read();

    // Setup the Controller Configuration Byte
    CONTROLLER.lock().write(0x20);
    while CONTROLLER.lock().read() & 0x1 == 0 {}
    let mut config_byte: u8 = DEVICE.lock().read();
    println!("Old Controller Configuration Byte: {:x}", config_byte);
    // TODO: see if two ports exist

    // Disable all IRQs and disable translation
    config_byte &= !(1 << 0);
    config_byte &= !(1 << 1);
    config_byte &= !(1 << 6);

    // write new configuration
    CONTROLLER.lock().write(0x60);
    while CONTROLLER.lock().read() & 0x2 == 1 {}
    DEVICE.lock().write(config_byte);
    println!("New Controller Configuration Byte: {:x}\n", config_byte);

    // Perform Controller Self Test
    CONTROLLER.lock().write(0xAA);
    while CONTROLLER.lock().read() & 0x1 == 0 {}
    assert!(DEVICE.lock().read() == 0x55, "Self check failed!");

    // Perform interface tests
    CONTROLLER.lock().write(0xAB);
    while CONTROLLER.lock().read() & 0x1 == 0 {}
    assert!(DEVICE.lock().read() == 0, "Interface test failed!");

    // Enable device(s)
    CONTROLLER.lock().write(0xAE);

    // Reset devices
    CONTROLLER.lock().write(0x20);
    while CONTROLLER.lock().read() & 0x1 == 0 {}
    let mut enable: u8 = DEVICE.lock().read();
    println!("Old Controller Configuration Byte: {:x}", enable);

    // Enable all IRQs
    enable |= 1 << 0;

    // write new configuration
    CONTROLLER.lock().write(0x60);
    while CONTROLLER.lock().read() & 0x2 == 1 {}
    DEVICE.lock().write(enable);
    println!("New Controller Configuration Byte: {:x}\n", enable);
    DEVICE.lock().read();
}

pub fn key_read() -> u8 {
    DEVICE.lock().read()
}
