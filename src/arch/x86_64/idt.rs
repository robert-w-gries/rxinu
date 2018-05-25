use arch::x86_64::gdt::{Descriptor, Gdt};
use arch::x86_64::interrupts::{exception, irq};
use spin::Once;
use x86_64::structures::idt::Idt;
use x86_64::structures::tss::TaskStateSegment;

static GDT: Once<Gdt> = Once::new();
static TSS: Once<TaskStateSegment> = Once::new();

const IRQ_OFFSET: usize = 32;
#[allow(dead_code)]
const SYSCALL_OFFSET: usize = 0x80;

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();

        idt.divide_by_zero.set_handler_fn(exception::divide_by_zero);
        idt.debug.set_handler_fn(exception::debug);
        idt.non_maskable_interrupt.set_handler_fn(exception::non_maskable_interrupt);
        idt.breakpoint.set_handler_fn(exception::breakpoint);
        idt.overflow.set_handler_fn(exception::overflow);
        idt.bound_range_exceeded.set_handler_fn(exception::bound_range_exceeded);
        idt.invalid_opcode.set_handler_fn(exception::invalid_opcode);
        idt.device_not_available.set_handler_fn(exception::device_not_available);
        idt.double_fault.set_handler_fn(exception::double_fault);
        idt.invalid_tss.set_handler_fn(exception::invalid_tss);
        idt.segment_not_present.set_handler_fn(exception::segment_not_present);
        idt.stack_segment_fault.set_handler_fn(exception::stack_segment_fault);
        idt.general_protection_fault.set_handler_fn(exception::general_protection_fault);
        idt.page_fault.set_handler_fn(exception::page_fault);
        idt.x87_floating_point.set_handler_fn(exception::x87_floating_point);
        idt.alignment_check.set_handler_fn(exception::alignment_check);
        idt.machine_check.set_handler_fn(exception::machine_check);
        idt.simd_floating_point.set_handler_fn(exception::simd_floating_point);
        idt.virtualization.set_handler_fn(exception::virtualization);
        idt.security_exception.set_handler_fn(exception::security_exception);

        idt[IRQ_OFFSET + 0].set_handler_fn(irq::timer);
        idt[IRQ_OFFSET + 1].set_handler_fn(irq::keyboard);
        idt[IRQ_OFFSET + 2].set_handler_fn(irq::cascade);
        idt[IRQ_OFFSET + 3].set_handler_fn(irq::com2);
        idt[IRQ_OFFSET + 4].set_handler_fn(irq::com1);

        // TODO: Syscall
        //idt[SYSCALL_OFFSET] = syscall_handler_entry(syscall::syscall);

        idt
    };
}

pub fn init() {
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;
    use x86_64::structures::gdt::SegmentSelector;

    let tss = TSS.call_once(|| {
        let tss = TaskStateSegment::new();
        // TODO: Why is this missing now?
        // tss.interrupt.stack_table[DOUBLE_FAULT_IST_INDEX] = VirtAddr::new()
        tss
    });

    let mut code_selector = SegmentSelector(0);
    let mut tss_selector = SegmentSelector(0);
    let gdt = GDT.call_once(|| {
        let mut gdt = Gdt::new();
        code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        tss_selector = gdt.add_entry(Descriptor::tss_segment(&tss));
        gdt
    });

    gdt.load();

    unsafe {
        set_cs(code_selector);
        load_tss(tss_selector);
    }

    IDT.load();
}
