use core::fmt;
use x86::shared::irq::{InterruptDescription, EXCEPTIONS};

macro_rules! exception {
    ($e:ident, $desc:expr, $func:block) => {
        pub extern "x86-interrupt" fn $e(stack_frame: &mut ExceptionStackFrame) {
            kprintln!("\n{:#?}\n{:#?}",
                      stack_frame,
                      $desc);
            $func
        }
    };
    ($e:ident, $desc:expr, $err_type:ty, $func:block) => {
        #[cfg(target_arch = "x86")]
        pub extern "x86-interrupt" fn $e(stack_frame: &mut ErrorExceptionStackFrame) {
            kprintln!("\n{:#?}\n{:#?}",
                      stack_frame,
                      $desc);
            $func
        }
        #[cfg(target_arch = "x86_64")]
        pub extern "x86-interrupt" fn $e(stack_frame: &mut ExceptionStackFrame, error_code: $err_type) {
            kprintln!("\nError code: {:#?}\n{:#?}\n{:#?}",
                      error_code,
                      stack_frame,
                      $desc);
            $func
        }
    };
}

// TODO: Implement actual error handling for each exception
exception!(divide_by_zero, EXCEPTIONS[0], {
    loop {}
});

exception!(debug, EXCEPTIONS[1], {
});

exception!(non_maskable, EXCEPTIONS[2], {
});

exception!(break_point, EXCEPTIONS[3], {
});

exception!(overflow, EXCEPTIONS[4], {
    loop {}
});

exception!(bound_range, EXCEPTIONS[5], {
    loop {}
});

exception!(invalid_opcode, EXCEPTIONS[6], {
    loop {}
});

exception!(device_not_available, EXCEPTIONS[7], {
    loop {}
});

exception!(double_fault, EXCEPTIONS[8], ErrorCode, {
    loop{}
});

exception!(invalid_tss, EXCEPTIONS[10], ErrorCode, {
    loop {}
});

exception!(segment_not_present, EXCEPTIONS[11], ErrorCode, {
    loop {}
});

exception!(stack_segment, EXCEPTIONS[12], ErrorCode, {
    loop {}
});

exception!(protection, EXCEPTIONS[13], ErrorCode, {
    loop {}
});

exception!(page_fault, EXCEPTIONS[14], PageFaultErrorCode, {
    use x86::shared::control_regs;
    kprintln!("\nPage fault while accessing {:#x}", unsafe { control_regs::cr2() });
    loop {}
});

exception!(fpu, EXCEPTIONS[16], {
    loop {}
});

exception!(alignment_check, EXCEPTIONS[17], ErrorCode, {
    loop {}
});

exception!(machine_check, EXCEPTIONS[18], {
    loop {}
});

exception!(simd, EXCEPTIONS[19], {
    loop {}
});

exception!(virtualization, EXCEPTIONS[20], {
    loop {}
});

exception!(security, SECURITY, ErrorCode, {
    loop {}
});

/// Source: AMD Secure Virtual Machine Architecture Reference Manual
pub static SECURITY: InterruptDescription = InterruptDescription {
    vector: 30,
    mnemonic: "#SX",
    description: "Security",
    irqtype: "",
    source: "Security sensitive events",
};

/// Represents the exception stack frame pushed by the CPU on exception entry.
#[cfg(target_arch = "x86")]
#[repr(C)]
pub struct ExceptionStackFrame {
    pub instruction_pointer: u32,
    pub code_segment: u32,
    pub cpu_flags: u32,
    pub stack_pointer: u32,
    pub stack_segment: u32,
}

#[cfg(target_arch = "x86")]
#[repr(C)]
pub struct ErrorExceptionStackFrame {
    pub error_code: u32,
    pub instruction_pointer: u32,
    pub code_segment: u32,
    pub cpu_flags: u32,
    pub stack_pointer: u32,
    pub stack_segment: u32,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct ExceptionStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

impl fmt::Debug for ExceptionStackFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[cfg(target_arch = "x86")]
        struct Hex(u32);
        #[cfg(target_arch = "x86_64")]
        struct Hex(u64);
        impl fmt::Debug for Hex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:#x}", self.0)
            }
        }

        let mut s = f.debug_struct("ExceptionStackFrame");
        s.field("instruction_pointer", &Hex(self.instruction_pointer));
        s.field("code_segment", &self.code_segment);
        s.field("cpu_flags", &Hex(self.cpu_flags));
        s.field("stack_pointer", &Hex(self.stack_pointer));
        s.field("stack_segment", &self.stack_segment);
        s.finish()
    }
}

#[cfg(target_arch = "x86")]
impl fmt::Debug for ErrorExceptionStackFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Hex(u32);
        impl fmt::Debug for Hex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:#x}", self.0)
            }
        }

        let mut s = f.debug_struct("ExceptionStackFrame");
        s.field("error_code", &self.error_code);
        s.field("instruction_pointer", &Hex(self.instruction_pointer));
        s.field("code_segment", &self.code_segment);
        s.field("cpu_flags", &Hex(self.cpu_flags));
        s.field("stack_pointer", &Hex(self.stack_pointer));
        s.field("stack_segment", &self.stack_segment);
        s.finish()
    }
}

#[derive(Debug)]
pub struct ErrorCode(u64);

bitflags! {
    /// Describes an page fault error code.
    pub struct PageFaultErrorCode: u64 {
        /// If this flag is set, the page fault was caused by a page-protection violation,
        /// else the page fault was caused by a not-present page.
        const PROTECTION_VIOLATION = 1 << 0;

        /// If this flag is set, the memory access that caused the page fault was a write.
        /// Else the access that caused the page fault is a memory read. This bit does not
        /// necessarily indicate the cause of the page fault was a read or write violation.
        const CAUSED_BY_WRITE = 1 << 1;

        /// If this flag is set, an access in user mode (CPL=3) caused the page fault. Else
        /// an access in supervisor mode (CPL=0, 1, or 2) caused the page fault. This bit
        /// does not necessarily indicate the cause of the page fault was a privilege violation.
        const USER_MODE = 1 << 2;

        /// If this flag is set, the page fault is a result of the processor reading a 1 from
        /// a reserved field within a page-translation-table entry.
        const MALFORMED_TABLE = 1 << 3;

        /// If this flag is set, it indicates that the access that caused the page fault was an
        /// instruction fetch.
        const INSTRUCTION_FETCH = 1 << 4;
    }
}
