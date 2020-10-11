use crate::device::pic_8259::{MAIN, WORKER};
use x86_64::registers::rflags::{self, RFlags};

pub mod exception;
pub mod irq;
pub mod syscall;

pub const DOUBLE_FAULT_IST_INDEX: usize = 0;

/// Disable interrupts
#[inline(always)]
pub unsafe fn disable() {
    llvm_asm!("cli" : : : : "intel", "volatile");
}

/// Enable interrupts
#[inline(always)]
pub unsafe fn enable() {
    llvm_asm!("sti; nop" : : : : "intel", "volatile");
}

pub fn enabled() -> bool {
    rflags::read().contains(RFlags::INTERRUPT_FLAG)
}

pub fn disable_then_execute<F, T>(uninterrupted_fn: F) -> T
where
    F: FnOnce() -> T,
{
    let interrupts_enabled = enabled();
    if interrupts_enabled == true {
        unsafe {
            disable();
        }
    }

    let result: T = uninterrupted_fn();

    if interrupts_enabled == true {
        unsafe {
            enable();
        }
    }

    result
}

/// Mask interrupts, execute code uninterrupted, restore original interrupts
pub fn mask_then_restore<F, T>(uninterrupted_fn: F) -> T
where
    F: FnOnce() -> T,
{
    let saved_masks: (u8, u8) = mask();
    let result: T = uninterrupted_fn();
    restore_mask(saved_masks);
    result
}

/// Mask interrupts then return tuple of previous state for PIC1 and PIC2
pub fn mask() -> (u8, u8) {
    unsafe {
        disable();
    }

    unsafe {
        let saved_mask1 = MAIN.lock().data.read();
        let saved_mask2 = WORKER.lock().data.read();
        MAIN.lock().data.write(0xff);
        WORKER.lock().data.write(0xff);
        (saved_mask1, saved_mask2)
    }
}

/// Unmask all interrupts
pub fn clear_mask() {
    unsafe {
        disable();
    }

    // Clear all masks from interrupt line so that all interrupts fire
    unsafe {
        MAIN.lock().data.write(0);
        WORKER.lock().data.write(0);
    }

    unsafe {
        enable();
    }
}

/// Enable interrupts, restoring the previously set masks
pub fn restore_mask(saved_masks: (u8, u8)) {
    unsafe {
        disable();
    }

    let (saved_mask1, saved_mask2) = saved_masks;

    unsafe {
        MAIN.lock().data.write(saved_mask1);
        WORKER.lock().data.write(saved_mask2);
    }

    unsafe {
        enable();
    }
}

#[inline(always)]
pub unsafe fn halt() {
    llvm_asm!("hlt");
}

#[inline(always)]
pub fn pause() {
    unsafe {
        llvm_asm!("pause");
    }
}

#[test_case]
fn breakpoint_exception() {
    unsafe {
        llvm_asm!("int3");
    }
}
