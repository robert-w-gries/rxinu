impl Context {
    pub fn set_page_table(&mut self, address: usize) {
        self.cr3 = address;
    }

    pub fn set_stack(&mut self, address: usize) {
        self.reg_sp = address;
    }
}

#[cfg(target_arch = "x86")]
mod bits32;
#[cfg(target_arch = "x86_64")]
mod bits64;

#[cfg(target_arch = "x86")]
pub use self::bits32::*;
#[cfg(target_arch = "x86_64")]
pub use self::bits64::*;
