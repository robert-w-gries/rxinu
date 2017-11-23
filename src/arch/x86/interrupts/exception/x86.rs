use core::fmt;

pub type ExceptionStack = ExceptionStack32;
pub type ErrorStack = ErrorStack32;

#[repr(C)]
/// Represents the exception stack frame pushed by the CPU on exception entry.
pub struct ExceptionStack32 {
    pub instruction_pointer: u32,
    pub code_segment: u32,
    pub cpu_flags: u32,
    pub stack_pointer: u32,
    pub stack_segment: u32,
}

#[repr(C)]
/// The exception stack frame contains an error code for certain exceptions.
pub struct ErrorStack32 {
    pub error_code: u32,
    pub instruction_pointer: u32,
    pub code_segment: u32,
    pub cpu_flags: u32,
    pub stack_pointer: u32,
    pub stack_segment: u32,
}

impl fmt::Debug for ErrorStack32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = f.debug_struct("ExceptionStack");
        s.field("error_code", &StackHex(self.error_code));
        s.field("instruction_pointer", &StackHex(self.instruction_pointer));
        s.field("code_segment", &StackHex(self.code_segment));
        s.field("cpu_flags", &StackHex(self.cpu_flags));
        s.field("stack_pointer", &StackHex(self.stack_pointer));
        s.field("stack_segment", &StackHex(self.stack_segment));
        s.finish()
    }
}

pub struct StackHex(pub u32);
