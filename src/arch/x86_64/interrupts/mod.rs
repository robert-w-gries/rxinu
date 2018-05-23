use device::pic_8259::{MASTER, SLAVE};
use syscall::io::Io;

pub mod exception;
pub mod irq;
pub mod syscall;

pub const DOUBLE_FAULT_IST_INDEX: usize = 0;

/// Disable interrupts, execute code uninterrupted, restore original interrupts
pub fn disable_then_restore<F, T>(uninterrupted_fn: F) -> T
where
    F: FnOnce() -> T,
{
    let saved_masks: (u8, u8) = disable();
    let result: T = uninterrupted_fn();
    restore(saved_masks);
    result
}

/// Disable interrupts then return tuple of previous state for PIC1 and PIC2
pub fn disable() -> (u8, u8) {
    unsafe {
        ::x86::shared::irq::disable();
    }

    let saved_mask1 = MASTER.lock().data.read();
    let saved_mask2 = SLAVE.lock().data.read();

    MASTER.lock().data.write(0xff);
    SLAVE.lock().data.write(0xff);

    (saved_mask1, saved_mask2)
}

/// Enable all interrupts
pub fn enable() {
    unsafe {
        ::x86::shared::irq::disable();
    }

    // Clear all masks from interrupt line so that all interrupts fire
    MASTER.lock().data.write(0);
    SLAVE.lock().data.write(0);

    unsafe {
        ::x86::shared::irq::enable();
    }
}

/// Enable interrupts, restoring the previously set masks
pub fn restore(saved_masks: (u8, u8)) {
    unsafe {
        ::x86::shared::irq::disable();
    }

    let (saved_mask1, saved_mask2) = saved_masks;

    MASTER.lock().data.write(saved_mask1);
    SLAVE.lock().data.write(saved_mask2);

    unsafe {
        ::x86::shared::irq::enable();
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn breakpoint_exception() {
        unsafe {
            asm!("int3");
        }
    }
}
