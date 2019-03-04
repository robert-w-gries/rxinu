#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ({
            #[cfg(feature = "vga")]
            {
                use core::fmt::Write;
                use crate::device::vga::VGA;

                let _ = VGA.lock().write_fmt(format_args!($($arg)*));
            }
    });
}

#[macro_export]
macro_rules! kprintln {
    () => (kprint!("\n"));
    ($fmt:expr) => (kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (kprint!(concat!($fmt, "\n"), $($arg)*));
}

pub fn clear_screen() {
    #[cfg(feature = "serial")]
    {
        use crate::device::uart_16550::COM1;
        COM1.lock().clear_screen();
    }

    #[cfg(feature = "vga")]
    {
        use crate::device::vga::VGA;
        VGA.lock().clear_screen();
    }
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => ({
            #[cfg(feature = "serial")]
            {
                use core::fmt::Write;
                use $crate::device::uart_16550::COM1;

                let _ = COM1.lock().write_fmt(format_args!($($arg)*));
            }
    });
}

#[macro_export]
macro_rules! serial_println {
    () => (serial_print!("\n"));
    ($fmt:expr) => (serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (serial_print!(concat!($fmt, "\n"), $($arg)*));
}
