#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ({
        use arch::interrupts;
        interrupts::disable_then_restore(|| {
            #[cfg(feature = "serial")]
            {
                use core::fmt::Write;
                use $crate::device::uart_16550::COM1;

                // ignore write result
                let _  = write!(COM1.lock(), $($arg)*);
            }

            #[cfg(feature = "vga")]
            {
                use core::fmt::Write;
                use $crate::device::vga::VGA;

                // ignore write result
                let _ = write!(VGA.lock(), $($arg)*);
            }
        });
    });
}

#[macro_export]
macro_rules! kprintln {
    () => (kprint!("\n"));
    ($fmt:expr) => (kprint!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (kprint!(concat!($fmt, "\n"), $($arg)*));
}

pub fn clear_screen() {
    use arch::interrupts;

    #[cfg(feature = "serial")]
    {
        use device::uart_16550::COM1;
        interrupts::disable_then_restore(|| {
            COM1.lock().clear_screen();
        });
    }

    #[cfg(feature = "vga")]
    {
        use device::vga::VGA;
        interrupts::disable_then_restore(|| {
            VGA.lock().clear_screen();
        });
    }
}
