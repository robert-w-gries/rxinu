use x86_64::structures::idt::{Idt, ExceptionStackFrame};
use interrupts::{exception, DOUBLE_FAULT_IST_INDEX};

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();

        idt.divide_by_zero.set_handler_fn(exception::divide_by_zero);
        idt.breakpoint.set_handler_fn(exception::breakpoint);
        idt.invalid_opcode.set_handler_fn(exception::invalid_opcode);
        idt.page_fault.set_handler_fn(exception::page_fault);

        unsafe {
            idt.double_fault.set_handler_fn(exception::double_fault)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX as u16);
        }

        idt
    };
}

pub fn init() {
    IDT.load();
}
