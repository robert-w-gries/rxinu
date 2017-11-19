use core::fmt::{self, Write};
use spin::Mutex;
use syscall::io::{Io, Port, ReadOnly};

const SERIAL_PORT1: u16 = 0x3F8;
const SERIAL_PORT2: u16 = 0x2F8;

pub static COM1: Mutex<SerialPort<Port<u8>>> =
    Mutex::new(SerialPort::<Port<u8>>::new(SERIAL_PORT1));
pub static COM2: Mutex<SerialPort<Port<u8>>> =
    Mutex::new(SerialPort::<Port<u8>>::new(SERIAL_PORT2));

// TODO: Replace arbitrary value for clearing rows
const BUF_MAX_HEIGHT: usize = 25;

pub fn init() {
    COM1.lock().init();
    COM2.lock().init();

    kprintln!("[ OK ] Serial Driver");
}

impl<T: Io<Value = u8>> Write for SerialPort<T> {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for byte in s.bytes() {
            self.send(byte);
        }
        Ok(())
    }
}

#[allow(dead_code)]
pub struct SerialPort<T: Io<Value = u8>> {
    data: T,
    int_en: T,
    fifo_ctrl: T,
    line_ctrl: T,
    modem_ctrl: T,
    line_sts: ReadOnly<T>,
    modem_sts: ReadOnly<T>,
}

impl SerialPort<Port<u8>> {
    pub const fn new(base: u16) -> SerialPort<Port<u8>> {
        SerialPort {
            data: Port::new(base),
            int_en: Port::new(base + 1),
            fifo_ctrl: Port::new(base + 2),
            line_ctrl: Port::new(base + 3),
            modem_ctrl: Port::new(base + 4),
            line_sts: ReadOnly::new(Port::new(base + 5)),
            modem_sts: ReadOnly::new(Port::new(base + 6)),
        }
    }
}

impl<T: Io<Value = u8>> SerialPort<T> {
    pub fn clear_screen(&mut self) {
        for _ in 0..BUF_MAX_HEIGHT {
            self.send(b'\n')
        }
    }

    pub fn init(&mut self) {
        self.int_en.write(0x00); // disable interrupts
        self.line_ctrl.write(0x80); // enable DLAB (set baud rate divisor)
        self.data.write(0x03); // set divisor to 3 (lo byte) 38400 baud
        self.int_en.write(0x00); // (hi byte)
        self.line_ctrl.write(0x03); // 8 bits, no parity, one stop bit
        self.fifo_ctrl.write(0xC7); // enable fifo, clear them, 14 byte threshold
        self.modem_ctrl.write(0x0B); // IRQs enabled, RTS/DSR set
        self.int_en.write(0x01); // enable interrupts
    }

    fn line_sts(&self) -> LineStsFlags {
        LineStsFlags::from_bits_truncate(self.line_sts.read())
    }

    pub fn receive(&mut self) -> u8 {
        // TODO: implement a buffer so we don't lose data
        let mut data: u8 = 0x0;
        while self.line_sts().contains(DATA_READY) {
            data = self.data.read();
        }
        data
    }

    pub fn send(&mut self, data: u8) {
        let mut wait_then_write = |data: u8| {
            while !self.line_sts().contains(THR_EMPTY) {}
            self.data.write(data);
        };

        match data {
            // backspace or delete
            0x8 | 0x7F => {
                wait_then_write(0x8);
                wait_then_write(b' ');
                wait_then_write(0x8);
            }
            b'\r' | b'\n' => {
                wait_then_write(b'\n');
                wait_then_write(b'\r');
            }
            _ => {
                wait_then_write(data);
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
        // 1 to 4 unknown
        const THR_EMPTY =     1 << 5;
        const TRANS_EMPTY =   1 << 6;
        // 6 and 7 unknown
    }
}
