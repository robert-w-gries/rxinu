use crate::syscall::io::{Io, Port};
use spin::Mutex;

pub static MASTER: Mutex<Pic> = Mutex::new(Pic::new(0x20));
pub static SLAVE: Mutex<Pic> = Mutex::new(Pic::new(0xA0));

pub fn init() {
    // We need to add a delay between writes to our PICs, especially on
    // older motherboards.  But we don't necessarily have any kind of
    // timers yet, because most of them require interrupts.  Various
    // older versions of Linux and other PC operating systems have
    // worked around this by writing garbage data to port 0x80, which
    // allegedly takes long enough to make everything work on most
    // hardware.  Port 0x80 is used for checkpoints during POST.
    // The Linux kernel seems to think it is free for use
    let mut wait_port: Port<u8> = Port::new(0x80);

    let mut write_then_wait = |mut port: Port<u8>, data: u8| {
        port.write(data);
        wait_port.write(0);
    };

    let saved_mask1 = MASTER.lock().data.read();
    let saved_mask2 = SLAVE.lock().data.read();

    // Start initialization
    let init_value: u8 = (ICW1::INIT as u8) + (ICW1::ICW4_NOT_NEEDED as u8);
    write_then_wait(MASTER.lock().cmd, init_value);
    write_then_wait(SLAVE.lock().cmd, init_value);

    // Set offsets
    write_then_wait(MASTER.lock().data, 0x20);
    write_then_wait(SLAVE.lock().data, 0x28);

    // Set up cascade (chaining between MASTER and SLAVE)
    write_then_wait(MASTER.lock().data, 0x4);
    write_then_wait(SLAVE.lock().data, 0x2);

    // Set up interrupt mode (1 is 8086/88 mode, 2 is auto EOI)
    write_then_wait(MASTER.lock().data, ICW4::MODE_8086 as u8);
    write_then_wait(SLAVE.lock().data, ICW4::MODE_8086 as u8);

    // Restore saved masks
    write_then_wait(MASTER.lock().data, saved_mask1);
    write_then_wait(SLAVE.lock().data, saved_mask2);

    kprintln!("[ OK ] PIC Driver");
}

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
