use x86::bits64::irq::*;
use x86_64::structures::idt::ExceptionStackFrame;

pub extern "x86-interrupt" fn divide_by_zero(stack_frame: &mut ExceptionStackFrame) {
    println!("\nEXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn breakpoint( stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn invalid_opcode(stack_frame: &mut ExceptionStackFrame) {
    println!("\nEXCEPTION: INVALID OPCODE at {:#x}\n{:#?}",
             stack_frame.instruction_pointer,
             stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn double_fault(
    stack_frame: &mut ExceptionStackFrame, _error_code: u64)
{
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop{}
}

pub extern "x86-interrupt" fn page_fault(stack_frame: &mut ExceptionStackFrame, error_code: PageFaultError) {
    use x86::shared::control_regs;
    println!("\nEXCEPTION: PAGE FAULT while accessing {:#x}\nerror code: \
                                  {:?}\n{:#?}",
             control_regs::cr2(),
             error_code,
             stack_frame);
    loop {}
}
