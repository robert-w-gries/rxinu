use core::mem;
use x86_64::registers::rflags::RFlags;

global_asm!(include_str!("context_switch.asm"));

extern "C" {
    fn x86_64_context_switch(prev: *mut Context, next: *const Context);
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Context {
    rflags: usize,
    rdi: usize,
    rsi: usize,
    rdx: usize,
    rcx: usize,
    r8: usize,
    r9: usize,
    rbx: usize,
    r12: usize,
    r13: usize,
    r14: usize,
    r15: usize,
    rbp: usize,
    rsp: usize,
}

impl Context {
    pub const fn empty() -> Context {
        Context {
            rflags: 0,
            rdi: 0,
            rsi: 0,
            rdx: 0,
            rcx: 0,
            r8: 0,
            r9: 0,
            rbx: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rbp: 0,
            rsp: 0,
        }
    }

    pub fn new(stack_top: *mut u8, proc_entry: usize, proc: usize) -> Context {
        let mut ctx = Context {
            rflags: RFlags::INTERRUPT_FLAG.bits() as usize,
            rdi: proc,
            rsi: 0,
            rdx: 0,
            rcx: 0,
            r8: 0,
            r9: 0,
            rbx: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rbp: stack_top as _,
            rsp: stack_top as usize,
        };

        unsafe {
            ctx.push_stack(proc_entry);
        }

        ctx
    }

    /// Push an item onto the stack.
    pub unsafe fn push_stack(&mut self, item: usize) {
        self.rsp -= mem::size_of::<usize>();
        *(self.rsp as *mut usize) = item;
    }

    #[inline]
    pub unsafe fn switch_to(&mut self, next: &Context) {
        x86_64_context_switch(self as *mut _, next as *const _);
    }
}
