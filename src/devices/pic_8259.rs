use syscall::io::{Io, Port};

pub struct Pic {
    pub cmd: Port<u8>,
    pub data: Port<u8>,
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
pub enum ICW1 {
    ICW4_NOT_NEEDED = 0x01,
    SINGLE_CASCADE_MODE = 0x02,
    INTERVAL4 = 0x04,
    LEVEL_TRIGGERED_MODE = 0x08,
    INIT = 0x10,
}

/// Initialization Command Word 4
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum ICW4 {
    MODE_8086 = 0x01,
    AUTO_EOI = 0x02,
    BUF_SLAVE = 0x08,
    BUF_MASTER = 0x0C,
    SFNM = 0x10,
}
