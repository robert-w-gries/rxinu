use core::mem;

extern "C" {
    fn x86_64_context_switch(prev: *mut Context, next: *const Context);
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Context {
    rflags: usize,
    cr3: usize,
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
            cr3: 0,
            rbx: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rbp: 0,
            rsp: 0,
        }
    }

    pub fn new(stack_top: *mut u8, proc_entry: usize) -> Context {
        let mut ctx = Context {
            rflags: 0,
            cr3: 0,
            rbx: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rbp: stack_top as _,
            rsp: stack_top as usize,
        };

        unsafe {
            ctx.push_stack(::task::process::process_ret as usize);
            ctx.push_stack(proc_entry);
        }

        ctx
    }

    /// Push an item onto the stack.
    pub unsafe fn push_stack(&mut self, item: usize) {
        self.rsp -= mem::size_of::<usize>();
        *(self.rsp as *mut usize) = item;
    }

    pub fn set_page_table(&mut self, address: usize) {
        self.cr3 = address;
    }

    pub fn set_stack(&mut self, address: usize) {
        self.rsp = address;
    }

    pub fn set_base_pointer(&mut self, address: usize) {
        self.rbp = address;
    }

    #[inline]
    pub unsafe extern "C" fn switch_to(&mut self, next: &mut Context) {
        x86_64_context_switch(self as *mut _, next as *const _);
    }
}
