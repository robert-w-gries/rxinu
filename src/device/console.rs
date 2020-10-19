#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ({
            #[cfg(feature = "vga")]
            {
                use core::fmt::Write;
                let _ = $crate::device::vga::VGA.lock().write_fmt(format_args!($($arg)*));
            }
    });
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::kprint!("\n"));
    ($fmt:expr) => ($crate::kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::kprint!(concat!($fmt, "\n"), $($arg)*));
}

pub fn clear_screen() {
    #[cfg(feature = "serial")]
    {
        use crate::device::serial::uart_16550::COM1;
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
                let _ = $crate::device::serial::uart_16550::COM1.lock().write_fmt(format_args!($($arg)*));
            }
    });
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}
