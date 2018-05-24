use arch::x86_64::interrupts;
use core::fmt;
use x86::shared::irq::{InterruptDescription
use x86_64::structures::idt::{Idt, ExceptionStackFrame};

// TODO: Implement actual error handling for each exception
extern "x86-interrupt" fn divide_by_zero(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Divide By Zero\n{:#?}", stack_frame);
    loop{}
}

extern "x86-interrupt" fn debug(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Debug\n{:#?}", stack_frame);
    loop{}
}

extern "x86-interrupt" fn non_maskable(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Non-maskable External Interrupt\n{:#?}", stack_frame);
    loop{}
}

extern "x86-interrupt" fn break_point(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Breakpoint\n{:#?}", stack_frame);
    loop{}
}

extern "x86-interrupt" fn overflow(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Overflow\n{:#?}", stack_frame);
    loop{}
}

extern "x86-interrupt" fn bound_range(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: BOUND Range Exceeded\n{:#?}", stack_frame);
    loop{}
}

extern "x86-interrupt" fn invalid_opcode(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Invalid Opcode\n{:#?}", stack_frame);
    loop{}
}

extern "x86-interrupt" fn device_not_available(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Device Not Available\n{:#?}", stack_frame);
    loop{}
}

extern "x86-interrupt" fn double_fault(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Double Fault\n{:#?}", stack_frame);
    loop{}
}

extern "x86-interrupt" fn invalid_tss(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Invalid TSS\n{:#?}", stack_frame);
    loop{}
}

extern "x86-interrupt" fn segment_not_present(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Segment Not Present\n{:#?}", stack_frame);
    loop{}
}

extern "x86-interrupt" fn stack_segment(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Stack Segment Fault\n{:#?}", stack_frame);
    loop{}
}

extern "x86-interrupt" fn protection(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: General Protection\n{:#?}", stack_frame);
    loop{}
}

extern "x86-interrupt" fn page_fault(stack_frame: &mut ExceptionStackFrame, error_code: PageFaultErrorCode) {
    use x86_64::registers::control_regs;
    kprintln!(
        "\nException: PAGE FAULT while accessing {:#x}\nerror code: \
         {:?}\n{:#?}",
        control_regs::cr2(),
        error_code,
        stack_frame
    );
    loop {}
}

extern "x86-interrupt" fn fpu(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: FPU Floating-Point\n{:#?}", stack_frame);
    loop{}
}

extern "x86-interrupt" fn alignment_check(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Alignment Check\n{:#?}", stack_frame);
    loop{}
}

extern "x86-interrupt" fn machine_check(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Machine Check\n{:#?}", stack_frame);
    loop{}
}

extern "x86-interrupt" fn simd(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: SIMD Floating-Point\n{:#?}", stack_frame);
    loop{}
}

extern "x86-interrupt" fn virtualization(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Virtualization\n{:#?}", stack_frame);
    loop{}
}

extern "x86-interrupt" fn security(stack_frame: &mut ExceptionStackFrame) {
    kprintln!("\nException: Security\n{:#?}", stack_frame);
    loop{}
}
