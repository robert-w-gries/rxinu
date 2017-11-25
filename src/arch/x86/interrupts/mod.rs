pub mod exception;
pub mod irq;
pub mod syscall;

pub const DOUBLE_FAULT_IST_INDEX: usize = 0;

/// Disable interrupts, execute code uninterrupted, re-enable interrupts
pub fn disable_interrupts_then<T>(uninterrupted_fn: T) -> ()
where
    T: FnOnce() -> (),
{
    unsafe {
        ::x86::shared::irq::disable();
    }
    uninterrupted_fn();
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
