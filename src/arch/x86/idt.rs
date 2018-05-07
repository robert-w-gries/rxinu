use arch::x86::interrupts::{irq, syscall, DOUBLE_FAULT_IST_INDEX};
use core::mem;
use x86::shared::dtables::{self, DescriptorTablePointer};
use x86::shared::paging::VAddr;
use x86::shared::segmentation::SegmentSelector;
use x86::shared::PrivilegeLevel;

#[cfg(target_arch = "x86")]
use x86::bits32::irq::IdtEntry;
#[cfg(target_arch = "x86_64")]
use x86::bits64::irq::{IdtEntry, Type};

const IRQ_OFFSET: usize = 32;
const SYSCALL_OFFSET: usize = 0x80;

const KERNEL_CODE_SELECTOR: SegmentSelector = SegmentSelector::new(1, PrivilegeLevel::Ring0);

lazy_static! {
    static ref IDT: [IdtEntry; 256] = {
        use arch::x86::interrupts::exception::*;

        let mut idt: [IdtEntry; 256] = [IdtEntry::MISSING; 256];

        idt[0] = intr_handler_entry(divide_by_zero as usize);
        idt[1] = intr_handler_entry(debug as usize);
        idt[2] = intr_handler_entry(non_maskable as usize);
        idt[3] = intr_handler_entry(break_point as usize);
        idt[4] = intr_handler_entry(overflow as usize);
        idt[5] = intr_handler_entry(bound_range as usize);
        idt[6] = intr_handler_entry(invalid_opcode as usize);
        idt[7] = intr_handler_entry(device_not_available as usize);
        idt[8] = double_fault_handler_entry(double_fault as usize, DOUBLE_FAULT_IST_INDEX as u8);
        // 9 no longer available
        idt[10] = intr_handler_entry(invalid_tss as usize);
        idt[11] = intr_handler_entry(segment_not_present as usize);
        idt[12] = intr_handler_entry(stack_segment as usize);
        idt[13] = intr_handler_entry(protection as usize);
        idt[14] = intr_handler_entry(page_fault as usize);
        // 15 reserved
        idt[16] = intr_handler_entry(fpu as usize);
        idt[17] = intr_handler_entry(alignment_check as usize);
        idt[18] = intr_handler_entry(machine_check as usize);
        idt[19] = intr_handler_entry(simd as usize);
        idt[20] = intr_handler_entry(virtualization as usize);
        // 21 through 29 reserved
        idt[30] = intr_handler_entry(security as usize);
        // 31 reserved

        idt[IRQ_OFFSET + 0] = intr_handler_entry(irq::timer as usize);
        idt[IRQ_OFFSET + 1] = intr_handler_entry(irq::keyboard as usize);
        idt[IRQ_OFFSET + 2] = intr_handler_entry(irq::cascade as usize);
        idt[IRQ_OFFSET + 3] = intr_handler_entry(irq::com2 as usize);
        idt[IRQ_OFFSET + 4] = intr_handler_entry(irq::com1 as usize);

        idt[SYSCALL_OFFSET] = syscall_handler_entry(syscall::syscall as usize);

        idt
    };
}

pub fn init() {
    let idtr: DescriptorTablePointer<IdtEntry> = DescriptorTablePointer {
        limit: (IDT.len() * mem::size_of::<IdtEntry>() - 1) as u16,
        base: IDT.as_ptr(),
    };

    unsafe {
        dtables::lidt(&idtr);
    }
}

#[cfg(target_arch = "x86")]
fn double_fault_handler_entry(ptr: usize, _index: u8) -> IdtEntry {
    intr_handler_entry(ptr)
}

#[cfg(target_arch = "x86_64")]
fn double_fault_handler_entry(ptr: usize, index: u8) -> IdtEntry {
    let mut i = intr_handler_entry(ptr);
    i.ist_index = index as u8;
    i
}

fn intr_handler_entry(ptr: usize) -> IdtEntry {
    create_idt_entry(ptr, PrivilegeLevel::Ring0)
}

fn syscall_handler_entry(ptr: usize) -> IdtEntry {
    create_idt_entry(ptr, PrivilegeLevel::Ring3)
}

#[cfg(target_arch = "x86")]
fn create_idt_entry(ptr: usize, privilege: PrivilegeLevel) -> IdtEntry {
    IdtEntry::new(
        VAddr::from_usize(ptr),
        KERNEL_CODE_SELECTOR.bits() as u16,
        privilege,
        true,
    )
}

#[cfg(target_arch = "x86_64")]
fn create_idt_entry(ptr: usize, privilege: PrivilegeLevel) -> IdtEntry {
    IdtEntry::new(
        VAddr::from_usize(ptr),
        KERNEL_CODE_SELECTOR,
        privilege,
        Type::InterruptGate,
        0,
    )
}
