use devices::pic_8259::Pic;
use spin::Mutex;
use syscall::io::{Io, Port};

pub static MASTER: Mutex<Pic> = Mutex::new(Pic::new(0x20));
pub static SLAVE: Mutex<Pic> = Mutex::new(Pic::new(0xA0));

pub fn init() {
    use devices::pic_8259::{ICW1, ICW4};

    // We need to add a delay between writes to our PICs, especially on
    // older motherboards.  But we don't necessarily have any kind of
    // timers yet, because most of them require interrupts.  Various
    // older versions of Linux and other PC operating systems have
    // worked around this by writing garbage data to port 0x80, which
    // allegedly takes long enough to make everything work on most
    // hardware.  Here, `wait` is a closure.
    let mut wait_port: Port<u8> = Port::new(0x80);

    let mut write_then_wait = |mut port: Port<u8>, data: u8| {
        port.write(data);
        wait_port.write(0);
    };

    //let saved_mask1 = MASTER.lock().data.read();
    //let saved_mask2 = SLAVE.lock().data.read();

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
    //write_then_wait(MASTER.lock().data, 0x0);
    //write_then_wait(SLAVE.lock().data, 0x0);
    //write_then_wait(MASTER.lock().data, saved_mask1);
    //write_then_wait(SLAVE.lock().data, saved_mask2);

    kprintln!("[ OK ] PIC Driver");
}
