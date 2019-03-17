use core::fmt;
use x86_64::structures::idt::InterruptStackFrame;

#[repr(C, packed)]
/// Represents the syscall stack
pub struct SyscallStack {
    pub fs: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
}

impl fmt::Debug for SyscallStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct StackHex(u64);
        impl fmt::Debug for StackHex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:#x}", self.0)
            }
        }
        let mut s = f.debug_struct("SyscallStack");
        s.field("fs", &StackHex(self.fs));
        s.field("r11", &StackHex(self.r11));
        s.field("r10", &StackHex(self.r10));
        s.field("r9", &StackHex(self.r9));
        s.field("r8", &StackHex(self.r8));
        s.field("rsi", &StackHex(self.rsi));
        s.field("rdi", &StackHex(self.rdi));
        s.field("rdx", &StackHex(self.rdx));
        s.field("rcx", &StackHex(self.rcx));
        s.field("rip", &StackHex(self.rip));
        s.field("cs", &StackHex(self.cs));
        s.field("rflags", &StackHex(self.rflags));
        s.finish()
    }
}

pub extern "x86-interrupt" fn syscall(_stack_frame: &mut InterruptStackFrame) {
    kprintln!("This is a syscall!");
    // TODO: syscall::match_syscall(eax_register, stack_frame.rsp);
}
