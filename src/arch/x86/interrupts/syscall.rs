use core::fmt;
use super::exception::ExceptionStack;

#[cfg(target_arch = "x86")]
type Bits = u32;
#[cfg(target_arch = "x86_64")]
type Bits = u64;

#[repr(C, packed)]
/// Represents the syscall stack
pub struct SyscallStack {
    pub fs: Bits,
    pub r11: Bits,
    pub r10: Bits,
    pub r9: Bits,
    pub r8: Bits,
    pub rsi: Bits,
    pub rdi: Bits,
    pub rdx: Bits,
    pub rcx: Bits,
    pub rip: Bits,
    pub cs: Bits,
    pub rflags: Bits,
}

impl fmt::Debug for SyscallStack{
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

pub extern "x86-interrupt" fn syscall(_stack_frame: &mut ExceptionStack) {
    kprintln!("This is a syscall!");
    // TODO: syscall::match_syscall(eax_register, stack_frame.rsp);
}
