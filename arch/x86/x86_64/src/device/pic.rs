use spin::Mutex;
use super::io::Io;
use super::io::port::Port;

pub static MASTER: Mutex<Pic> = Mutex::new(Pic::new(0x20));
pub static SLAVE: Mutex<Pic> = Mutex::new(Pic::new(0xA0));

pub fn init() {
    use self::ICW1::{INIT, ICW4_NOT_NEEDED};
    use self::ICW4::MODE_8086;

    // Start initialization
    MASTER.lock().cmd.write((INIT as u8) + (ICW4_NOT_NEEDED as u8));
    SLAVE.lock().cmd.write((INIT as u8) + (ICW4_NOT_NEEDED as u8));

    // Set offsets
    MASTER.lock().data.write(0x20);
    SLAVE.lock().data.write(0x28);

    // Set up cascade
    MASTER.lock().data.write(4);
    SLAVE.lock().data.write(2);

    // Set up interrupt mode (1 is 8086/88 mode, 2 is auto EOI)
    MASTER.lock().data.write(MODE_8086 as u8);
    SLAVE.lock().data.write(MODE_8086 as u8);

    // Unmask interrupts
    MASTER.lock().data.write(0);
    SLAVE.lock().data.write(0);

    // Ack remaining interrupts
    MASTER.lock().ack();
    SLAVE.lock().ack();
}

pub struct Pic {
    cmd: Port<u8>,
    data: Port<u8>,
}

impl Pic {
    pub const fn new(port: u16) -> Pic {
        Pic {
            cmd: Port::new(port),
            data: Port::new(port + 1),
        }
    }

    pub fn ack(&mut self) {
        self.cmd.write(0x20);
    }

    pub fn mask_set(&mut self, irq: u8) {
        assert!(irq < 8);

        let mut mask = self.data.read();
        mask |= 1 << irq;
        self.data.write(mask);
    }

    pub fn mask_clear(&mut self, irq: u8) {
        assert!(irq < 8);

        let mut mask = self.data.read();
        mask &= !(1 << irq);
        self.data.write(mask);
    }
}

/// Initialization Command Word 1
#[allow(dead_code)]
#[allow(non_camel_case_types)]
enum ICW1 {
    ICW4_NOT_NEEDED = 0x01,
    SINGLE_CASCADE_MODE = 0x02,
    INTERVAL4 = 0x04,
    LEVEL_TRIGGERED_MODE = 0x08,
    INIT = 0x10,
}

/// Initialization Command Word 4
#[allow(dead_code)]
#[allow(non_camel_case_types)]
enum ICW4 {
    MODE_8086 = 0x01,
    AUTO_EOI = 0x02,
    BUF_SLAVE = 0x08,
    BUF_MASTER = 0x0C,
    SFNM = 0x10,
}
