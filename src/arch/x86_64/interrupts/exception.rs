use arch::x86_64::{self, interrupts};
use x86_64::structures::idt::{ExceptionStackFrame, PageFaultErrorCode};

macro_rules! exception {
    ($x:ident, $stack:ident, $func:block) => {
        pub extern "x86-interrupt" fn $x($stack: &mut ExceptionStackFrame) {
            interrupts::disable_then_restore(|| $func);
        }
    };
    ($x:ident, $stack:ident, $err:ident, $func:block) => {
        pub extern "x86-interrupt" fn $x($stack: &mut ExceptionStackFrame, $err: u64) {
            interrupts::disable_then_restore(|| $func);
        }
    };
    ($x:ident, $stack:ident, $err:ident, $err_type:ty, $func:block) => {
        pub extern "x86-interrupt" fn $x($stack: &mut ExceptionStackFrame, $err: $err_type) {
            interrupts::disable_then_restore(|| $func);
        }
    };
}

// TODO: Implement actual error handling for each exception
exception!(divide_by_zero, _stack, {
    kprintln!("Divide By Zero Fault");
});

exception!(debug, _stack, {
    kprintln!("Debug Trap");
});

exception!(non_maskable_interrupt, _stack, {
    kprintln!("Non-maskable Interrupt");
});

exception!(breakpoint, _stack, {
    kprintln!("Breakpoint trap");
});

exception!(overflow, _stack, {
    kprintln!("Overflow trap");
});

exception!(bound_range_exceeded, _stack, {
    kprintln!("Bound Range Exceeded Fault");
});

exception!(invalid_opcode, _stack, {
    kprintln!("Invalid Opcode Fault");
    loop {
        unsafe {
            x86_64::halt();
        }
    }
});

exception!(device_not_available, _stack, {
    kprintln!("Device Not Available Fault");
});

exception!(double_fault, stack, error, {
    kprintln!("Double Fault: {}|{:#?}", error, stack);
});

exception!(invalid_tss, _stack, _error, {
    kprintln!("Invalid TSS Fault");
});

exception!(segment_not_present, stack, error, {
    kprintln!("Segment Not Present Fault: 0x{:x}\n{:#?}", error, stack);
});

exception!(stack_segment_fault, _stack, _error, {
    kprintln!("Stack Segment Fault");
});

exception!(general_protection_fault, _stack, _error, {
    kprintln!("General Protection Fault");
    loop {
        unsafe {
            x86_64::halt();
        }
    }
});

exception!(page_fault, stack, err, PageFaultErrorCode, {
    let cr2: u64 = unsafe {
        let reg: u64;
        asm!("mov %cr2, $0" : "=r"(reg));
        reg
    };

    kprintln!("\nPage fault while accessing {:#x}\nError Code: {:?}\n{:#?}",
              cr2,
              err,
              stack);

    loop {
        unsafe {
            x86_64::halt();
        }
    }
});

exception!(x87_floating_point, _stack, {
    kprintln!("x87 Floating Point Exception");
});

exception!(alignment_check, _stack, _error, {
    kprintln!("Alignment Check Fault");
});

exception!(machine_check, _stack, {
    kprintln!("Machine Check Abort");
});

exception!(simd_floating_point, _stack, {
    kprintln!("SIMD Floating Point Exception");
});

exception!(virtualization, _stack, {
    kprintln!("Virtualization Exception");
});

exception!(security_exception, _stack, _error, {
    kprintln!("Security Exception");
});
