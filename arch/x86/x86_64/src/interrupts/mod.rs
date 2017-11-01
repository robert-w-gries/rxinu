use x86::bits64::task::TaskStateSegment;

use memory::MemoryController;

mod exception;
mod gdt;
mod idt;
mod irq;

const DOUBLE_FAULT_IST_INDEX: usize = 0;

/// Initialize double fault stack and load gdt and idt 
pub fn init(memory_controller: &mut MemoryController) {

    let mut tss = TaskStateSegment::new();

    let double_fault_stack = memory_controller.alloc_stack(1)
        .expect("could not allocate double fault stack");
    tss.ist[DOUBLE_FAULT_IST_INDEX] = double_fault_stack.top() as u64;

    gdt::init(&tss);
    unsafe { idt::init(); }

    // TODO: Fix interrupt handling
    // unsafe { asm!("int3"); }
}

#[cfg(test)]
mod tests {

    #[test]
    fn breakpoint_exception() {
        ::x86::shared::irq::int!(3);
    }
}
