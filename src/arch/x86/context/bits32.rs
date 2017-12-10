#[derive(Clone, Debug)]
pub struct Context {
    pub cr3: usize,
    reg_bp: usize,
    reg_flags: usize,
    pub reg_sp: usize,
    ebx: usize,
    edi: usize,
    esi: usize,
}

impl Context {
    pub fn new() -> Context {
        Context {
            cr3: 0,
            reg_flags: 0,
            ebx: 0,
            edi: 0,
            esi: 0,
            reg_bp: 0,
            reg_sp: 0,
        }
    }

    #[naked]
    #[inline(never)]
    /// The arguments are ignored because the actual values are on the stack
    /// This function requires complete control over function stack layout, so we use naked function
    /// Safety: This function is entirely assembly code, so it is inherently unsafe
    pub unsafe extern "C" fn switch_to(&mut self, _next: &mut Context) {
        let old: &mut Context;
        let new: &mut Context;

        asm!("mov $0, 4[esp]" : "=r"(old) : : "memory" : "intel", "volatile");
        asm!("mov $0, 8[esp]" : "=r"(new) : : "memory" : "intel", "volatile");

        asm!("pushfd ; pop $0" : "=r"(old.reg_flags) : : "memory" : "intel", "volatile");
        asm!("push $0 ; popfd" : : "r"(new.reg_flags) : "memory" : "intel", "volatile");

        asm!("mov $0, cr3" : "=r"(old.cr3) : : "memory" : "intel", "volatile");
        asm!("mov $0, ebx" : "=r"(old.ebx) : : "memory" : "intel", "volatile");
        asm!("mov $0, edi" : "=r"(old.edi) : : "memory" : "intel", "volatile");
        asm!("mov $0, esi" : "=r"(old.esi) : : "memory" : "intel", "volatile");
        asm!("mov $0, esp" : "=r"(old.reg_sp) : : "memory" : "intel", "volatile");
        asm!("mov $0, ebp" : "=r"(old.reg_bp) : : "memory" : "intel", "volatile");

        if new.cr3 != old.cr3 {
            asm!("mov cr3, $0" : : "r"(new.cr3) : "memory" : "intel", "volatile");
        }
        asm!("mov ebx, $0" : : "r"(new.ebx) : "memory" : "intel", "volatile");
        asm!("mov edi, $0" : : "r"(new.edi) : "memory" : "intel", "volatile");
        asm!("mov esi, $0" : : "r"(new.esi) : "memory" : "intel", "volatile");
        asm!("mov esp, $0" : : "r"(new.reg_sp) : "memory" : "intel", "volatile");
        asm!("mov ebp, $0" : : "r"(new.reg_bp) : "memory" : "intel", "volatile");
    }
}
