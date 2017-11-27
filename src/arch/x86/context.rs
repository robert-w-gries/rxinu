use scheduling::process::Process;

pub struct Context {
    cr3: usize,
    reg_flags: usize,
    reg_bx: usize,
    reg_12: usize,
    reg_13: usize,
    reg_14: usize,
    reg_15: usize,
    reg_bp: usize,
    reg_sp: usize,
}

impl Context {
    pub fn new() -> Context {
        Context {
            cr3: 0,
            reg_flags: 0,
            reg_bx: 0,
            reg_12: 0,
            reg_13: 0,
            reg_14: 0,
            reg_15: 0,
            reg_bp: 0,
            reg_sp: 0,
        }
    }

    pub fn set_page_table(&mut self, address: usize) {
        self.cr3 = address;
    }

    pub fn set_stack(&mut self, address: usize) {
        self.reg_sp = address;
    }

    //#[naked]
    pub unsafe fn switch_to(&mut self, next: &mut Context) {
        asm!("mov $0, cr3" : "=r"(self.cr3) : : "memory" : "intel", "volatile");
        asm!("mov $0, rbx" : "=r"(self.reg_bx) : : "memory" : "intel", "volatile");
        asm!("mov $0, r12" : "=r"(self.reg_12) : : "memory" : "intel", "volatile");
        asm!("mov $0, r13" : "=r"(self.reg_13) : : "memory" : "intel", "volatile");
        asm!("mov $0, r14" : "=r"(self.reg_14) : : "memory" : "intel", "volatile");
        asm!("mov $0, r15" : "=r"(self.reg_15) : : "memory" : "intel", "volatile");
        asm!("mov $0, rsp" : "=r"(self.reg_sp) : : "memory" : "intel", "volatile");
        asm!("mov $0, rbp" : "=r"(self.reg_bp) : : "memory" : "intel", "volatile");

        if next.cr3 != self.cr3 {
            asm!("mov cr3, $0" : : "r"(next.cr3) : "memory" : "intel", "volatile");
        }
        asm!("mov rbx, $0" : : "r"(next.reg_bx) : "memory" : "intel", "volatile");
        asm!("mov r12, $0" : : "r"(next.reg_12) : "memory" : "intel", "volatile");
        asm!("mov r13, $0" : : "r"(next.reg_13) : "memory" : "intel", "volatile");
        asm!("mov r14, $0" : : "r"(next.reg_14) : "memory" : "intel", "volatile");
        asm!("mov r15, $0" : : "r"(next.reg_15) : "memory" : "intel", "volatile");
        asm!("mov rsp, $0" : : "r"(next.reg_sp) : "memory" : "intel", "volatile");
        asm!("mov rbp, $0" : : "r"(next.reg_bp) : "memory" : "intel", "volatile");
    }
}
