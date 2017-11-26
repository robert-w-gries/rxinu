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

    #[naked]
    pub fn switch_to(&mut self, next: &mut Process) {
    }
}
