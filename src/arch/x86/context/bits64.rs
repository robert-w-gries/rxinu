#[derive(Clone, Debug)]
pub struct Context {
    pub cr3: usize,
    pub reg_bp: usize,
    pub reg_flags: usize,
    pub reg_sp: usize,
    pub rbx: usize,
    pub r12: usize,
    pub r13: usize,
    pub r14: usize,
    pub r15: usize,
}

impl Context {
    pub fn new() -> Context {
        Context {
            cr3: 0,
            reg_flags: 0,
            rbx: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            reg_bp: 0,
            reg_sp: 0,
        }
    }

    #[naked]
    #[inline(never)]
    pub unsafe extern "C" fn switch_to(&mut self, next: &mut Context) {
        asm!("pushfq ; pop $0" : "=r"(self.reg_flags) : : "memory" : "intel", "volatile");
        asm!("push $0 ; popfq" : : "r"(next.reg_flags) : "memory" : "intel", "volatile");

        // save CPU state
        asm!("mov $0, cr3" : "=r"(self.cr3) : : "memory" : "intel", "volatile");
        asm!("mov $0, rbx" : "=r"(self.rbx) : : "memory" : "intel", "volatile");
        asm!("mov $0, r12" : "=r"(self.r12) : : "memory" : "intel", "volatile");
        asm!("mov $0, r13" : "=r"(self.r13) : : "memory" : "intel", "volatile");
        asm!("mov $0, r14" : "=r"(self.r14) : : "memory" : "intel", "volatile");
        asm!("mov $0, r15" : "=r"(self.r15) : : "memory" : "intel", "volatile");
        asm!("mov $0, rsp" : "=r"(self.reg_sp) : : "memory" : "intel", "volatile");
        asm!("mov $0, rbp" : "=r"(self.reg_bp) : : "memory" : "intel", "volatile");

        // load new CPU state
        if next.cr3 != self.cr3 {
            asm!("mov cr3, $0" : : "r"(next.cr3) : "memory" : "intel", "volatile");
        }
        asm!("mov rbx, $0" : : "r"(next.rbx) : "memory" : "intel", "volatile");
        asm!("mov r12, $0" : : "r"(next.r12) : "memory" : "intel", "volatile");
        asm!("mov r13, $0" : : "r"(next.r13) : "memory" : "intel", "volatile");
        asm!("mov r14, $0" : : "r"(next.r14) : "memory" : "intel", "volatile");
        asm!("mov r15, $0" : : "r"(next.r15) : "memory" : "intel", "volatile");
        asm!("mov rsp, $0" : : "r"(next.reg_sp) : "memory" : "intel", "volatile");
        asm!("mov rbp, $0" : : "r"(next.reg_bp) : "memory" : "intel", "volatile");
    }
}
