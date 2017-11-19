#[macro_use]
pub mod console;
mod device;
mod interrupts;
mod memory;

pub fn init(multiboot_information_address: usize) {
    let boot_info = unsafe { ::multiboot2::load(multiboot_information_address) };

    let mut memory_controller = memory::init(&boot_info);

    // Make sure interrupts don't intefere with setup
    interrupts::disable_interrupts_then(|| {
      interrupts::init(&mut memory_controller);
      device::init();
    });
}

#[allow(dead_code)]
fn enable_nxe_bit() {
    use x86::shared::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86::shared::control_regs::*;

    unsafe { cr0_write(cr0() | CR0_WRITE_PROTECT) };
}
