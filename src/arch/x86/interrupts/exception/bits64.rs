use core::fmt;

pub type ExceptionStack = ExceptionStack64;
pub type ErrorStack = ExceptionStack64;

#[repr(C)]
/// Represents the exception stack frame pushed by the CPU on exception entry.
pub struct ExceptionStack64 {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

pub struct StackHex(pub u64);

pub struct ErrorCode(u64);

impl fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0)
    }
}
