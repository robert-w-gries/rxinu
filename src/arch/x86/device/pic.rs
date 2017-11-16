use devices::pic_8259::Pic;
use spin::Mutex;
use syscall::io::Io;

pub static MASTER: Mutex<Pic> = Mutex::new(Pic::new(0x20));
pub static SLAVE: Mutex<Pic> = Mutex::new(Pic::new(0xA0));

pub fn init() {
    use devices::pic_8259::{ICW1, ICW4};

    // Start initialization
    MASTER.lock().cmd.write((ICW1::INIT as u8) + (ICW1::ICW4_NOT_NEEDED as u8));
    SLAVE.lock().cmd.write((ICW1::INIT as u8) + (ICW1::ICW4_NOT_NEEDED as u8));

    // Set offsets
    MASTER.lock().data.write(0x20);
    SLAVE.lock().data.write(0x28);

    // Set up cascade
    MASTER.lock().data.write(4);
    SLAVE.lock().data.write(2);

    // Set up interrupt mode (1 is 8086/88 mode, 2 is auto EOI)
    MASTER.lock().data.write(ICW4::MODE_8086 as u8);
    SLAVE.lock().data.write(ICW4::MODE_8086 as u8);

    // Unmask interrupts
    MASTER.lock().data.write(0);
    SLAVE.lock().data.write(0);

    // Ack remaining interrupts
    MASTER.lock().ack();
    SLAVE.lock().ack();
}
