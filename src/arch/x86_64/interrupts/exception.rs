use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

use crate::arch::x86_64::interrupts::halt;

macro_rules! exception {
    ($x:ident, $stack:ident, $func:block) => {
        pub extern "x86-interrupt" fn $x($stack: &mut InterruptStackFrame) {
            $func;
        }
    };
    ($x:ident, $stack:ident, $err:ident, $func:block) => {
        pub extern "x86-interrupt" fn $x($stack: &mut InterruptStackFrame, $err: u64) {
            $func;
        }
    };
    ($x:ident, $stack:ident, $err:ident, $err_type:ty, $func:block) => {
        pub extern "x86-interrupt" fn $x($stack: &mut InterruptStackFrame, $err: $err_type) {
            $func;
        }
    };
}

// TODO: Implement actual error handling for each exception
exception!(divide_by_zero, _stack, {
    kprintln!("\nDivide By Zero Fault");
});

exception!(debug, _stack, {
    kprintln!("\nDebug Trap");
});

exception!(non_maskable_interrupt, _stack, {
    kprintln!("\nNon-maskable Interrupt");
});

exception!(breakpoint, _stack, {
    kprintln!("\nBreakpoint trap");
});

exception!(overflow, _stack, {
    kprintln!("\nOverflow trap");
});

exception!(bound_range_exceeded, _stack, {
    kprintln!("\nBound Range Exceeded Fault");
});

exception!(invalid_opcode, _stack, {
    kprintln!("\nInvalid Opcode Fault");
    loop {
        unsafe {
            halt();
        }
    }
});

exception!(device_not_available, _stack, {
    kprintln!("\nDevice Not Available Fault");
});

exception!(double_fault, stack, _error, {
    kprintln!("\nDouble Fault: {:#?}", stack);
    loop {
        unsafe {
            halt();
        }
    }
});

exception!(invalid_tss, _stack, _error, {
    kprintln!("\nInvalid TSS Fault");
});

exception!(segment_not_present, stack, error, {
    kprintln!("\nSegment Not Present Fault: 0x{:x}\n{:#?}", error, stack);
});

exception!(stack_segment_fault, _stack, _error, {
    kprintln!("\nStack Segment Fault");
});

exception!(general_protection_fault, _stack, _error, {
    kprintln!("\nGeneral Protection Fault");
    loop {
        unsafe {
            halt();
        }
    }
});

exception!(page_fault, stack, err, PageFaultErrorCode, {
    let cr2: u64 = unsafe {
        let reg: u64;
        asm!("mov %cr2, $0" : "=r"(reg));
        reg
    };

    kprintln!(
        "\nPage fault while accessing {:#x}\nError Code: {:?}\n{:#?}",
        cr2,
        err,
        stack
    );

    loop {
        unsafe {
            halt();
        }
    }
});

exception!(x87_floating_point, _stack, {
    kprintln!("\nx87 Floating Point Exception");
});

exception!(alignment_check, _stack, _error, {
    kprintln!("\nAlignment Check Fault");
});

exception!(machine_check, _stack, {
    kprintln!("\nMachine Check Abort");
});

exception!(simd_floating_point, _stack, {
    kprintln!("\nSIMD Floating Point Exception");
});

exception!(virtualization, _stack, {
    kprintln!("\nVirtualization Exception");
});

exception!(security_exception, _stack, _error, {
    kprintln!("\nSecurity Exception");
});
