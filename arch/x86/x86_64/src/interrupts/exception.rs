//use x86::shared::irq::EXCEPTIONS;
use x86::bits64::irq::*;

use super::exception_stack_frame::ExceptionStackFrame;

extern "x86-interrupt" fn divide_by_zero(stack_frame: &mut ExceptionStackFrame) {
    println!("\nEXCEPTION: DIVIDE BY ZERO\n{:#?}", stack_frame);
    //println!("{:#?}", EXCEPTIONS[0]);
    loop {}
}

extern "x86-interrupt" fn breakpoint( stack_frame: &mut ExceptionStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    //println!("{:#?}", EXCEPTIONS[1]);
}

extern "x86-interrupt" fn invalid_opcode(stack_frame: &mut ExceptionStackFrame) {
    println!("\nEXCEPTION: INVALID OPCODE at {:#x}\n{:#?}",
             stack_frame.instruction_pointer,
             stack_frame);
    //println!("{:#?}", EXCEPTIONS[6]);
    loop {}
}

extern "x86-interrupt" fn double_fault(
    stack_frame: &mut ExceptionStackFrame, _error_code: u64)
{
    println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    //println!("{:#?}", EXCEPTIONS[8]);
    loop{}
}

extern "x86-interrupt" fn page_fault(stack_frame: &mut ExceptionStackFrame, error_code: PageFaultError) {
    use x86::shared::control_regs;
    println!("\nEXCEPTION: PAGE FAULT while accessing {:#x}\nerror code: \
                                  {:?}\n{:#?}",
             control_regs::cr2(),
             error_code,
             stack_frame);
    //println!("{:#?}", EXCEPTIONS[14]);
    loop {}
}

pub static EXCEPTIONS: [extern fn(); 32] = [
    divide_by_zero,
    (),   // TODO: debug
    (),   // TODO: non_maskable
    breakpoint,
    (),   // TODO: overflow
    (),   // TODO: bound_range
    invalid_opcode,
    (),   // TODO: device_not_available
    double_fault,
    (),   // Index 9: Not available
    (),   // TODO: invalid_tss
    (),   // TODO: segment_not_present
    (),   // TODO: stack_segment
    (),   // TODO: protection
    page_fault,
    // Index 15: Reserved
    (),
    (),   // TODO: fpu
    (),   // TODO: alignment_check
    (),   // TODO: machine_check
    (),   // TODO: simd
    (),   // TODO: virtualization 
    // Indices 21-29: reserved
    (), (), (), (), (), (), (), (), (),
    (),   // TODO: security
    // Index 31: Reserved
];
