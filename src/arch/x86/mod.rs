#[macro_use]
pub mod console;
mod device;
mod gdt;
mod idt;
mod interrupts;
mod memory;

pub fn init(multiboot_information_address: usize) {
    let boot_info = unsafe { ::multiboot2::load(multiboot_information_address) };

    let mut memory_controller = memory::init(&boot_info);

    gdt::init(&mut memory_controller);

    interrupts::disable_interrupts_then(|| {
        idt::init();
        device::init();
    });
}

#[cfg(target_arch = "x86_64")]
fn enable_nxe_bit() {
    use x86::shared::msr::{rdmsr, wrmsr, IA32_EFER};

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

use x86::shared::PrivilegeLevel;
use x86::shared::segmentation::SegmentSelector;

const USER_DATA: SegmentSelector =
    SegmentSelector::new(gdt::GDT_USER_DATA as u16, PrivilegeLevel::Ring3);
const USER_CODE: SegmentSelector =
    SegmentSelector::new(gdt::GDT_USER_CODE as u16, PrivilegeLevel::Ring3);

/// Enter usermode.
/// To enter Ring3, we must pretend to raise an inter-privilege level interrupt.
/// [unsafe]
/// This function is pure assembly and is inherently unsafe
#[allow(unreachable_code)]
pub unsafe fn enter_usermode(ip: usize, sp: usize) -> ! {
    use x86::shared::flags::{FLAGS_IOPL0, Flags, FLAGS_IF};

    gdt::load_selectors(gdt::GDT_USER_DATA, PrivilegeLevel::Ring3);

    // Setup stack
    asm!("push r10
        push r11
        push r12
        push r13
        push r14"
        : // No output
        : "{r10}"(USER_DATA.bits()),
          "{r11}"(sp),
          "{r12}"(Flags::new() | FLAGS_IOPL0 | FLAGS_IF), // Enable interrupts
          "{r13}"(USER_CODE.bits()),
          "{r15}"(ip)
        : // Doesn't matter because function does not return
        : "intel", "volatile");

    // execute interrupt return then execute in usermode
    execute_ring3_code();
    unreachable!();
}

#[cfg(target_arch = "x86")]
/// Execute interrupt return to enter userspace
unsafe fn execute_ring3_code() -> ! {
    asm!("iret");
    unreachable!();
}

#[cfg(target_arch = "x86_64")]
/// Execute interrupt return to enter userspace
unsafe fn execute_ring3_code() -> ! {
    asm!("iretq");
    unreachable!();
}
