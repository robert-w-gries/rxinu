use arch::x86::interrupts::idt::{ExceptionStackFrame, PageFaultErrorCode};

pub extern "x86-interrupt" fn divide_by_zero(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Divide by zero at {:#x}\n{:#?}",
             stack_frame.instruction_pointer,
             stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn debug(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Debug trap at {:#x}\n{:#?}",
              stack_frame.instruction_pointer,
              stack_frame);
}

pub extern "x86-interrupt" fn non_maskable(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Non-maskable interrupt at {:#x}\n{:#?}",
              stack_frame.instruction_pointer,
              stack_frame);
}

pub extern "x86-interrupt" fn breakpoint(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Breakpoint trap at {:#x}\n{:#?}",
              stack_frame.instruction_pointer,
              stack_frame);
}

pub extern "x86-interrupt" fn overflow(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Overflow trap at {:#x}\n{:#?}",
             stack_frame.instruction_pointer,
             stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn bound_range(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Bound range exceeded fault at {:#x}\n{:#?}",
             stack_frame.instruction_pointer,
             stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn invalid_opcode(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Invalid opcode fault at {:#x}\n{:#?}",
             stack_frame.instruction_pointer,
             stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn device_not_available(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Device not available fault at {:#x}\n{:#?}",
             stack_frame.instruction_pointer,
             stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn double_fault(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
    kprintln!("Exception: Double fault\n{:#?}", stack_frame);
    loop{}
}

pub extern "x86-interrupt" fn invalid_tss(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
    kprintln!("\nException: Invalid TSS fault at {:#x}\n{:#?}",
             stack_frame.instruction_pointer,
             stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn segment_not_present(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
    kprintln!("\nException: Segment not present fault at {:#x}\n{:#?}",
             stack_frame.instruction_pointer,
             stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn stack_segment(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
    kprintln!("\nException: Stack segment fault at {:#x}\n{:#?}",
             stack_frame.instruction_pointer,
             stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn protection(stack_frame: &mut ExceptionStackFrame, error_code: u64) {
    kprintln!("\nException: Protection fault at {:#x}\nError Code: {:#x}\n{:#?}",
             stack_frame.instruction_pointer,
             error_code,
             stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn page_fault(stack_frame: &mut ExceptionStackFrame, error_code: PageFaultErrorCode) {
    use x86::shared::control_regs;
    kprintln!("\nException: Page fault while accessing {:#x}\nerror code: {:?}\n{:#?}",
             unsafe { control_regs::cr2() },
             error_code,
             stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn fpu(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: FPU floating point fault at {:#x}\n{:#?}",
             stack_frame.instruction_pointer,
             stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn alignment_check(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
    kprintln!("\nException: Alignment check fault at {:#x}\n{:#?}",
             stack_frame.instruction_pointer,
             stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn machine_check(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Machine check fault at {:#x}\n{:#?}",
             stack_frame.instruction_pointer,
             stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn simd(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: SIMD floating point fault at {:#x}\n{:#?}",
             stack_frame.instruction_pointer,
             stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn virtualization(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Virtualization fault at {:#x}\n{:#?}",
             stack_frame.instruction_pointer,
             stack_frame);
    loop {}
}

pub extern "x86-interrupt" fn security(stack_frame: &mut ExceptionStackFrame, _error_code: u64) {
    kprintln!("\nException: Security exception at {:#x}\n{:#?}",
             stack_frame.instruction_pointer,
             stack_frame);
    loop {}
}
