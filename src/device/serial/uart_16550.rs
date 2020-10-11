use crate::sync::IrqLock;
use bitflags::bitflags;
use core::fmt;
use x86_64::instructions::port::Port;

const SERIAL_PORT1: u16 = 0x3F8;
const SERIAL_PORT2: u16 = 0x2F8;

pub static COM1: IrqLock<SerialPort> = IrqLock::new(SerialPort::new(SERIAL_PORT1));
pub static COM2: IrqLock<SerialPort> = IrqLock::new(SerialPort::new(SERIAL_PORT2));

// TODO: Replace arbitrary value for clearing rows
const BUF_MAX_HEIGHT: usize = 25;
const FIFO_BYTE_THRESHOLD: usize = 14;

bitflags! {
    /// Interrupt enable register flags
    pub struct IntEnFlags: u8 {
        const RECEIVED =        1 << 0;
        const SENT =            1 << 1;
        const ERRORED =         1 << 2;
        const STATUS_CHANGE =   1 << 3;
        // 4 to 7 are unused
    }
}

bitflags! {
    /// Line status flags
    pub struct LineStsFlags: u8 {
        const DATA_READY =    1 << 0;
        // 1 to 4 unknown
        const THR_EMPTY =     1 << 5;
        const TRANS_EMPTY =   1 << 6;
        // 6 and 7 unknown
    }
}

pub fn init() {
    unsafe {
        COM1.lock().init();
        COM2.lock().init();
    }

    kprintln!("[ OK ] Serial Driver");
}

fn get_fifo_ctrl_byte() -> u8 {
    let mut byte = 1 << 0; // enable FIFO
    byte |= 1 << 1; // clear Receive FIFO
    byte |= 1 << 2; // clear Transmit FIFO

    // bits 6-7 represent num bytes in interrupt trigger
    byte |= match FIFO_BYTE_THRESHOLD {
        1 => 0x0,
        4 => 0x1,
        8 => 0x2,
        14 => 0x3,
        _ => 0x3, // default to 14 bytes
    };

    byte
}

pub struct SerialPort {
    data: Port<u8>,
    int_en: Port<u8>,
    fifo_ctrl: Port<u8>,
    line_ctrl: Port<u8>,
    modem_ctrl: Port<u8>,
    line_sts: Port<u8>,
    _modem_sts: Port<u8>,
}

impl SerialPort {
    pub const fn new(base: u16) -> SerialPort {
        SerialPort {
            data: Port::new(base),
            int_en: Port::new(base + 1),
            fifo_ctrl: Port::new(base + 2),
            line_ctrl: Port::new(base + 3),
            modem_ctrl: Port::new(base + 4),
            line_sts: Port::new(base + 5),
            _modem_sts: Port::new(base + 6),
        }
    }
}

impl SerialPort {
    pub fn clear_screen(&mut self) {
        for _ in 0..BUF_MAX_HEIGHT {
            self.send(b'\n')
        }
    }

    pub unsafe fn init(&mut self) {
        self.int_en.write(0x00); // disable interrupts
        self.line_ctrl.write(0x80); // enable DLAB (set baud rate divisor)
        self.data.write(0x03); // set divisor to 3 (lo byte) 38400 baud
        self.int_en.write(0x00); // (hi byte)
        self.line_ctrl.write(0x03); // 8 bits, no parity, one stop bit

        // 16550 specific FIFO Control Register
        let fifo_byte = get_fifo_ctrl_byte();
        self.fifo_ctrl.write(fifo_byte);

        self.modem_ctrl.write(0x0B); // IRQs enabled, RTS/DSR set
        self.int_en.write(0x01); // enable interrupts
    }

    pub fn line_sts(&mut self) -> LineStsFlags {
        LineStsFlags::from_bits_truncate(unsafe { self.line_sts.read() })
    }

    pub fn receive(&mut self) -> u8 {
        unsafe { self.data.read() }
    }

    pub fn send(&mut self, data: u8) {
        let mut wait_then_write = |data: u8| {
            while !self.line_sts().contains(LineStsFlags::THR_EMPTY) {}
            unsafe {
                self.data.write(data);
            }
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

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.send(byte);
        }
        Ok(())
    }
}
