use arch::x86::interrupts::{DOUBLE_FAULT_IST_INDEX, irq};
use core::fmt;
use core::mem;
use x86::shared::PrivilegeLevel;
use x86::shared::dtables::{self, DescriptorTablePointer};
use x86::shared::paging::VAddr;
use x86::shared::segmentation::SegmentSelector;

#[cfg(target_arch = "x86")] use x86::bits32::irq::IdtEntry;
#[cfg(target_arch = "x86_64")] use x86::bits64::irq::{IdtEntry, Type};

const IRQ_OFFSET: usize = 32;
const KERNEL_CODE_SELECTOR: SegmentSelector = SegmentSelector::new(1, PrivilegeLevel::Ring0);

// TODO: change to lazy static
static mut IDT: [IdtEntry; 256] = [IdtEntry::MISSING; 256];

// TODO: change to non static mut
static mut IDTR: DescriptorTablePointer<IdtEntry> = DescriptorTablePointer {
    limit: 0,
    base: 0 as * const _,
};

pub unsafe fn init() {
    use arch::x86::interrupts::exception::*;

    IDTR.limit = (IDT.len() * mem::size_of::<IdtEntry>() - 1) as u16;
    IDTR.base = IDT.as_ptr();

    IDT[0] = fn_handler_entry(divide_by_zero as usize);
    IDT[1] = fn_handler_entry(debug as usize);
    IDT[2] = fn_handler_entry(non_maskable as usize);
    IDT[3] = fn_handler_entry(break_point as usize);
    IDT[4] = fn_handler_entry(overflow as usize);
    IDT[5] = fn_handler_entry(bound_range as usize);
    IDT[6] = fn_handler_entry(invalid_opcode as usize);
    IDT[7] = fn_handler_entry(device_not_available as usize);
    IDT[8] = double_fault_handler_entry(double_fault as usize,
                                        DOUBLE_FAULT_IST_INDEX as u8);
    // 9 no longer available
    IDT[10] = fn_handler_entry(invalid_tss as usize);
    IDT[11] = fn_handler_entry(segment_not_present as usize);
    IDT[12] = fn_handler_entry(stack_segment as usize);
    IDT[13] = fn_handler_entry(protection as usize);
    IDT[14] = fn_handler_entry(page_fault as usize);
    // 15 reserved
    IDT[16] = fn_handler_entry(fpu as usize);
    IDT[17] = fn_handler_entry(alignment_check as usize);
    IDT[18] = fn_handler_entry(machine_check as usize);
    IDT[19] = fn_handler_entry(simd as usize);
    IDT[20] = fn_handler_entry(virtualization as usize);
    // 21 through 29 reserved
    IDT[30] = fn_handler_entry(security as usize);
    // 31 reserved

    IDT[IRQ_OFFSET+0] = fn_handler_entry(irq::timer as usize);
    IDT[IRQ_OFFSET+1] = fn_handler_entry(irq::keyboard as usize);
    IDT[IRQ_OFFSET+2] = fn_handler_entry(irq::cascade as usize);
    IDT[IRQ_OFFSET+3] = fn_handler_entry(irq::com2 as usize);
    IDT[IRQ_OFFSET+4] = fn_handler_entry(irq::com1 as usize);

    dtables::lidt(&IDTR);
}

#[cfg(target_arch = "x86")]
fn fn_handler_entry(ptr: usize) -> IdtEntry {
    IdtEntry::new(VAddr::from_usize(ptr), KERNEL_CODE_SELECTOR.bits() as u16,
                  PrivilegeLevel::Ring0, true)
}

#[cfg(target_arch = "x86_64")]
fn fn_handler_entry(ptr: usize) -> IdtEntry {
    IdtEntry::new(VAddr::from_usize(ptr), KERNEL_CODE_SELECTOR,
                  PrivilegeLevel::Ring0, Type::InterruptGate, 0)
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
