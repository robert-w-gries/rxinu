use crate::syscall::io::{Io, Port};
use spin::Mutex;

/// Operate in channel 0. Use mode 3, and operate with lobyte/hibyte.
const PIT_SET: u8 = 0x36;
static DIVISOR: u16 = 2685;

pub static PIT: Mutex<[Port<u8>; 2]> = Mutex::new([
    // Command register.
    Port::new(0x43),
    // Channel 0.
    Port::new(0x40),
]);

pub fn init() {
    PIT.lock()[0].write(PIT_SET);
    // Lower 8 bytes.
    PIT.lock()[1].write((DIVISOR & 0xFF) as u8);
    PIT.lock()[1].write((DIVISOR >> 8) as u8);

    kprintln!("[ OK ] Programmable Interval Timer");
}
