use x86_64::registers::flags;
use device::pic_8259::{MASTER, SLAVE};
use syscall::io::Io;

pub mod exception;
pub mod irq;
pub mod syscall;

pub const DOUBLE_FAULT_IST_INDEX: usize = 0;

/// Disable interrupts
#[inline(always)]
pub unsafe fn asm_disable() {
    asm!("cli" : : : : "intel", "volatile");
}

/// Disable interrupts
#[inline(always)]
pub unsafe fn asm_enable() {
    asm!("sti; nop" : : : : "intel", "volatile");
}

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
        asm_disable();
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
        asm_disable();
    }

    // Clear all masks from interrupt line so that all interrupts fire
    MASTER.lock().data.write(0);
    SLAVE.lock().data.write(0);

    unsafe {
        asm_enable();
    }
}

pub fn enabled() -> bool {
    flags::flags().contains(flags::Flags::IF)
}

/// Enable interrupts, restoring the previously set masks
pub fn restore(saved_masks: (u8, u8)) {
    unsafe {
        asm_disable();
    }

    let (saved_mask1, saved_mask2) = saved_masks;

    MASTER.lock().data.write(saved_mask1);
    SLAVE.lock().data.write(saved_mask2);

    unsafe {
        asm_enable();
    }
}

#[inline(always)]
pub unsafe fn halt() {
    asm!("hlt");
}

#[inline(always)]
pub unsafe fn pause() {
    asm!("pause");
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
