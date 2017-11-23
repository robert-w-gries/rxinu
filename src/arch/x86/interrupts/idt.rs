use arch::x86::interrupts::{irq, DOUBLE_FAULT_IST_INDEX};
use core::mem;
use x86::shared::PrivilegeLevel;
use x86::shared::dtables::{self, DescriptorTablePointer};
use x86::shared::paging::VAddr;
use x86::shared::segmentation::SegmentSelector;

#[cfg(target_arch = "x86")]
use x86::bits32::irq::IdtEntry;
#[cfg(target_arch = "x86_64")]
use x86::bits64::irq::{IdtEntry, Type};

const IRQ_OFFSET: usize = 32;
const KERNEL_CODE_SELECTOR: SegmentSelector = SegmentSelector::new(1, PrivilegeLevel::Ring0);

lazy_static! {
    static ref IDT: [IdtEntry; 256] = {
        use arch::x86::interrupts::exception::*;

        let mut idt: [IdtEntry; 256] = [IdtEntry::MISSING; 256];

        idt[0] = fn_handler_entry(divide_by_zero as usize);
        idt[1] = fn_handler_entry(debug as usize);
        idt[2] = fn_handler_entry(non_maskable as usize);
        idt[3] = fn_handler_entry(break_point as usize);
        idt[4] = fn_handler_entry(overflow as usize);
        idt[5] = fn_handler_entry(bound_range as usize);
        idt[6] = fn_handler_entry(invalid_opcode as usize);
        idt[7] = fn_handler_entry(device_not_available as usize);
        idt[8] = double_fault_handler_entry(double_fault as usize, DOUBLE_FAULT_IST_INDEX as u8);
        // 9 no longer available
        idt[10] = fn_handler_entry(invalid_tss as usize);
        idt[11] = fn_handler_entry(segment_not_present as usize);
        idt[12] = fn_handler_entry(stack_segment as usize);
        idt[13] = fn_handler_entry(protection as usize);
        idt[14] = fn_handler_entry(page_fault as usize);
        // 15 reserved
        idt[16] = fn_handler_entry(fpu as usize);
        idt[17] = fn_handler_entry(alignment_check as usize);
        idt[18] = fn_handler_entry(machine_check as usize);
        idt[19] = fn_handler_entry(simd as usize);
        idt[20] = fn_handler_entry(virtualization as usize);
        // 21 through 29 reserved
        idt[30] = fn_handler_entry(security as usize);
        // 31 reserved

        idt[IRQ_OFFSET + 0] = fn_handler_entry(irq::timer as usize);
        idt[IRQ_OFFSET + 1] = fn_handler_entry(irq::keyboard as usize);
        idt[IRQ_OFFSET + 2] = fn_handler_entry(irq::cascade as usize);
        idt[IRQ_OFFSET + 3] = fn_handler_entry(irq::com2 as usize);
        idt[IRQ_OFFSET + 4] = fn_handler_entry(irq::com1 as usize);

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
fn fn_handler_entry(ptr: usize) -> IdtEntry {
    IdtEntry::new(
        VAddr::from_usize(ptr),
        KERNEL_CODE_SELECTOR.bits() as u16,
        PrivilegeLevel::Ring0,
        true,
    )
}

#[cfg(target_arch = "x86_64")]
fn fn_handler_entry(ptr: usize) -> IdtEntry {
    IdtEntry::new(
        VAddr::from_usize(ptr),
        KERNEL_CODE_SELECTOR,
        PrivilegeLevel::Ring0,
        Type::InterruptGate,
        0,
    )
}

#[cfg(target_arch = "x86")]
fn double_fault_handler_entry(ptr: usize, _index: u8) -> IdtEntry {
    fn_handler_entry(ptr)
}

#[cfg(target_arch = "x86_64")]
fn double_fault_handler_entry(ptr: usize, index: u8) -> IdtEntry {
    let mut i = fn_handler_entry(ptr);
    i.ist_index = index as u8;
    i
}
