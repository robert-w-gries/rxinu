use spin::Mutex;
use devices::uart_16550::SerialPort;
use syscall::io::Port;

const SERIAL_PORT1: u16 = 0x3F8;
const SERIAL_PORT2: u16 = 0x2F8;

pub static COM1: Mutex<SerialPort<Port<u8>>> = Mutex::new(SerialPort::<Port<u8>>::new(SERIAL_PORT1));
pub static COM2: Mutex<SerialPort<Port<u8>>> = Mutex::new(SerialPort::<Port<u8>>::new(SERIAL_PORT2));

pub fn init() {
    COM1.lock().init();
    COM2.lock().init();

    kprintln!("[ OK ] Serial Driver");
}
