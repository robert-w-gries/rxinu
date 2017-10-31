use x86::bits64
use spin::Once;

use memory::MemoryController;
use self::Idt;

mod gdt;
mod idt;
mod irq;

static TSS: Once<TaskStateSegment> = Once::new();
static GDT: Once<gdt::Gdt> = Once::new();

/// Initialize double fault stack and load gdt and idt 
#[inline(always)]
pub fn init(memory_controller: &mut MemoryController) {
    use x86::shared::gdt::SegmentSelector;
    use x86::shared::segmentation::set_cs;
    use x86::shared::tables::load_tss;

    gdt::init();
    idt::init();

    let double_fault_stack = memory_controller.alloc_stack(1)
        .expect("could not allocate double fault stack");

    let tss = TSS.call_once(|| {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX] = VirtualAddress(
            double_fault_stack.top());
        tss
    });

    let mut code_selector = SegmentSelector(0);
    let mut tss_selector = SegmentSelector(0);
    let gdt = GDT.call_once(|| {
        let mut gdt = gdt::Gdt::new();
        code_selector = gdt.add_entry(gdt::Descriptor::kernel_code_segment());
        tss_selector = gdt.add_entry(gdt::Descriptor::tss_segment(&tss));
        gdt
    });
    gdt.load();

    unsafe {
        // reload code segment register and load TSS
        set_cs(code_selector);
        load_tss(tss_selector);
    }

    IDT.load();

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
