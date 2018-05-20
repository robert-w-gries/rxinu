#[derive(Clone, Debug)]
#[repr(C)]
pub struct Context {
    pub reg_flags: usize,
    pub cr3: usize,
    pub rbx: usize,
    pub r12: usize,
    pub r13: usize,
    pub r14: usize,
    pub r15: usize,
    pub reg_bp: usize,
    pub reg_sp: usize,
}

impl Context {
    pub fn new() -> Context {
        Context {
            reg_flags: 0,
            cr3: 0,
            rbx: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            reg_bp: 0,
            reg_sp: 0,
        }
    }

    //#[naked]
    //#[inline(never)]
    pub unsafe extern "C" fn switch_to(&mut self, next: &mut Context) {
        x86_64_context_switch(self as *mut _, next as *const _);
    }
    //    asm!("pushfq ; pop $0" : "=r"(self.reg_flags) : : "memory" : "intel", "volatile");
    //    asm!("push $0 ; popfq" : : "r"(next.reg_flags) : "memory" : "intel", "volatile");

    //    // save CPU state
    //    asm!("mov $0, cr3" : "=r"(self.cr3) : : "memory" : "intel", "volatile");
    //    asm!("mov $0, rbx" : "=r"(self.rbx) : : "memory" : "intel", "volatile");
    //    asm!("mov $0, r12" : "=r"(self.r12) : : "memory" : "intel", "volatile");
    //    asm!("mov $0, r13" : "=r"(self.r13) : : "memory" : "intel", "volatile");
    //    asm!("mov $0, r14" : "=r"(self.r14) : : "memory" : "intel", "volatile");
    //    asm!("mov $0, r15" : "=r"(self.r15) : : "memory" : "intel", "volatile");
    //    asm!("mov $0, rsp" : "=r"(self.reg_sp) : : "memory" : "intel", "volatile");
    //    asm!("mov $0, rbp" : "=r"(self.reg_bp) : : "memory" : "intel", "volatile");

    //    // load new CPU state
    //    if next.cr3 != self.cr3 {
    //        asm!("mov cr3, $0" : : "r"(next.cr3) : "memory" : "intel", "volatile");
    //    }
    //    asm!("mov rbx, $0" : : "r"(next.rbx) : "memory" : "intel", "volatile");
    //    asm!("mov r12, $0" : : "r"(next.r12) : "memory" : "intel", "volatile");
    //    asm!("mov r13, $0" : : "r"(next.r13) : "memory" : "intel", "volatile");
    //    asm!("mov r14, $0" : : "r"(next.r14) : "memory" : "intel", "volatile");
    //    asm!("mov r15, $0" : : "r"(next.r15) : "memory" : "intel", "volatile");
    //    asm!("mov rsp, $0" : : "r"(next.reg_sp) : "memory" : "intel", "volatile");
    //    asm!("mov rbp, $0" : : "r"(next.reg_bp) : "memory" : "intel", "volatile");
    //}
}

global_asm!("
.global x86_64_context_switch
.intel_syntax noprefix
# ThreadContext {
#   0x0: flags
#   0x8: cr3
#   0x8: rbx
#   0x10: r12
#   0x18: r13
#   0x20: r14
#   0x28: r15
#   0x30: rbp
#   0x38: rsp
# }
#
# rdi <- reference to previous `ThreadContext`
# rsi <- reference to next `ThreadContext`
x86_64_context_switch:
    # Save the previous context
    pushfq
    pop qword ptr [rdi] # save rflags into prev.flags
    # Rust inline assembly error: invalid operand
    #mov [rdi+0x8], cr3  # save rbx
    mov [rdi+0x10], rbx  # save rbx
    mov [rdi+0x18], r12 # save r12
    mov [rdi+0x20], r13 # save r13
    mov [rdi+0x28], r14 # save r14
    mov [rdi+0x30], r15 # save r15
    mov [rdi+0x38], rbp # save rbp

    # Swap the stack pointers
    mov [rdi+0x40], rsp # save rsp
    mov rsp, [rsi+0x40] # set rsp

    # Switch to the next context
    mov rbp, [rsi+0x38] # set rbp
    mov r15, [rsi+0x30] # set r15
    mov r14, [rsi+0x28] # set r14
    mov r13, [rsi+0x20] # set r13
    mov r12, [rsi+0x18] # set r12
    mov rbx, [rsi+0x10]  # set rbx
    # Rust inline assembly error: invalid operand
    #mov cr3, [rsi+0x8]  # set rbx
    push [rsi] # set rflags
    popfq

    # leap of faith
    ret
");

extern "C" {
    fn x86_64_context_switch(prev: *mut Context, next: *const Context);
}
