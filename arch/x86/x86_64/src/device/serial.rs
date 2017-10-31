use core::fmt::{self, Write};
use spin::Mutex;

use super::io::{Io, ReadOnly};
use super::io::port::Port;

const MAX_HEIGHT: usize = 25;

pub static COM1: Mutex<SerialPort> = Mutex::new(SerialPort::new(0x3F8));
pub static COM2: Mutex<SerialPort> = Mutex::new(SerialPort::new(0x2F8));

impl Write for SerialPort {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for byte in s.bytes() {
            self.send(byte);
        }
        Ok(())
    }
}

pub fn init() {
    COM1.lock().init();
    COM2.lock().init();
}

#[allow(dead_code)]
pub struct SerialPort {
    data: Port<u8>,
    int_en: Port<u8>,
    fifo_ctrl: Port<u8>,
    line_ctrl: Port<u8>,
    modem_ctrl: Port<u8>,
    line_sts: ReadOnly<Port<u8>>,
    modem_sts: ReadOnly<Port<u8>>,
}

impl SerialPort {
    const fn new(base: u16) -> SerialPort {
        SerialPort {
            data: Port::new(base),
            int_en: Port::new(base + 1),
            fifo_ctrl: Port::new(base + 2),
            line_ctrl: Port::new(base + 3),
            modem_ctrl: Port::new(base + 4),
            line_sts: ReadOnly::new(Port::new(base + 5)),
            modem_sts: ReadOnly::new(Port::new(base + 6))
        }
    }

    pub fn clear_screen(&mut self) {
        for _ in 0..MAX_HEIGHT {
            self.send(b'\n')
        }
    }

    fn init(&mut self) {
        self.int_en.write(0x00);        // disable interrupts
        self.line_ctrl.write(0x80);     // enable DLAB (set baud rate divisor)
        self.data.write(0x03);          // set divisor to 3 (lo byte) 38400 baud
        self.int_en.write(0x00);        // (hi byte)
        self.line_ctrl.write(0x03);     // 8 bits, no parity, one stop bit
        self.fifo_ctrl.write(0xC7);     // enable fifo, clear them, 14 byte threshold
        self.modem_ctrl.write(0x0B);    // IRQs enabled, RTS/DSR set
        self.int_en.write(0x01);
    }

    fn line_sts(&self) -> LineStsFlags {
        LineStsFlags::from_bits_truncate(self.line_sts.read())
    }

    pub fn receive(&mut self) -> u8 {
        while self.line_sts().contains(DATA_READY) {}
        self.data.read()
    }

    pub fn send(&mut self, data: u8) {
        match data {
            /// backspace or delete
            8 | 0x7F => {
                while ! self.line_sts().contains(THR_EMPTY) {}
                self.data.write(8);
                while ! self.line_sts().contains(THR_EMPTY) {}
                self.data.write(b' ');
                while ! self.line_sts().contains(THR_EMPTY) {}
                self.data.write(8);
            },
            b'\n' => {
                while ! self.line_sts().contains(THR_EMPTY) {}
                self.data.write(data);
                while ! self.line_sts().contains(THR_EMPTY) {}
                self.data.write(b'\r');
            },
            _ => {
                while ! self.line_sts().contains(THR_EMPTY) {}
                self.data.write(data);
            }
        }
    }
}

bitflags! {
    /// Interrupt enable register flags
    struct IntEnFlags: u8 {
        const RECEIVED =        1 << 0;
        const SENT =            1 << 1;
        const ERRORED =         1 << 2;
        const STATUS_CHANGE =   1 << 3;
        // 4 to 7 are unused
    }
}

bitflags! {
    /// Line status flags
    struct LineStsFlags: u8 {
        const DATA_READY =    1 << 0;
        const THR_EMPTY =     1 << 5;
        const TRANS_EMPTY =   1 << 6;
    }
}
