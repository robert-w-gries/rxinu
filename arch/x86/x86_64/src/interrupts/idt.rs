use x86_64::structures::idt::Idt;
use interrupts::{self, exception, irq};

// TODO: Fix IDT by switching to x86 crate
lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();

        idt.divide_by_zero.set_handler_fn(exception::divide_by_zero);
        idt.breakpoint.set_handler_fn(exception::breakpoint);
        idt.invalid_opcode.set_handler_fn(exception::invalid_opcode);
        idt.page_fault.set_handler_fn(exception::page_fault);

        unsafe {
            idt.double_fault.set_handler_fn(exception::double_fault)
                .set_stack_index(interrupts::DOUBLE_FAULT_IST_INDEX as u16);
        }

        idt.interrupts[0].set_handler_fn(irq::cascade);
        idt.interrupts[1].set_handler_fn(irq::com1);
        idt.interrupts[2].set_handler_fn(irq::com2);

        idt
    };
}

pub fn init() {
    IDT.load();
}
