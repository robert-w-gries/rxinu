use spin::Once;
use x86::bits64::task::TaskStateSegment;

use memory::MemoryController;

mod exception;
mod gdt;
mod idt;
mod irq;

const DOUBLE_FAULT_IST_INDEX: usize = 0;

static TSS: Once<TaskStateSegment> = Once::new();

/// Initialize double fault stack and load gdt and idt 
#[inline(always)]
pub fn init(memory_controller: &mut MemoryController) {
    use x86::shared::segmentation::{set_cs, SegmentSelector};

    let double_fault_stack = memory_controller.alloc_stack(1)
        .expect("could not allocate double fault stack");

    let tss = TSS.call_once(|| {
        let mut tss = TaskStateSegment::new();
        tss.ist[DOUBLE_FAULT_IST_INDEX] = double_fault_stack.top() as u64;
        tss
    });

    gdt::init(tss);
    idt::init();

    println!("IT DID NOT CRASH!");
    println!("IT DID NOT CRASH!");
    println!("IT DID NOT CRASH!");
    println!("IT DID NOT CRASH!");
    println!("IT DID NOT CRASH!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn breakpoint_exception() {
        ::x86::shared::interrupts::int3();
    }
}
