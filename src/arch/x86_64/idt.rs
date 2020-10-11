use crate::arch::x86_64::interrupts::{exception, irq};
use crate::arch::x86_64::gdt;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

const IRQ_OFFSET: usize = 32;
#[allow(dead_code)]
const SYSCALL_OFFSET: usize = 0x80;

lazy_static! {
    static ref IDT: InterruptDescriptorTable  = {
        let mut idt = InterruptDescriptorTable::new();

        idt.divide_error.set_handler_fn(exception::divide_error);
        idt.debug.set_handler_fn(exception::debug);
        idt.non_maskable_interrupt.set_handler_fn(exception::non_maskable_interrupt);
        idt.breakpoint.set_handler_fn(exception::breakpoint);
        idt.overflow.set_handler_fn(exception::overflow);
        idt.bound_range_exceeded.set_handler_fn(exception::bound_range_exceeded);
        idt.invalid_opcode.set_handler_fn(exception::invalid_opcode);
        idt.device_not_available.set_handler_fn(exception::device_not_available);
        unsafe {
            idt.double_fault.set_handler_fn(exception::double_fault)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
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
    IDT.load();
}
