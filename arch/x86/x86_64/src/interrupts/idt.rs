use core::mem::size_of;

use x86::bits64::irq::IdtEntry;
use x86::shared::PrivilegeLevel;
use x86::shared::descriptor::Flags;
use x86::shared::dtables::DescriptorTablePointer;
use x86::shared::segmentation;

use super::exception::EXCEPTIONS;
use super::irq::IRQS;
use super::exception_stack_frame::ExceptionStackFrame;

pub static mut IDTR: DescriptorTablePointer<IdtEntry> = DescriptorTablePointer {
    limit: 0,
    base: IdtEntry::MISSING,
};

pub static mut IDT: [IdtEntry; 256] = [IdtEntry::MISSING, 256];

const IRQ_OFFSET: u8 = 32;

pub fn init() {
    IDTR.limit = (IDT.len() * size_of::<IdtEntry>() - 1) as u16;
    IDTR.base = IDT.as_ptr();

    for i in 0..EXCEPTIONS.len() {
        let e: extern fn() = EXCEPTIONS[i];
        if e == () { continue; }
        set_func(&mut IDT[i], e);
    }

    for i in 0..IRQS.len() {
        set_func(&mut IDT[IRQ_OFFSET+i], IRQS[i]);
    }

    dtables::lidt(&IDTR);
}

fn set_func(i: IdtEntry, e: fn()) {
    i.base_lo = ((e.as_usize() as u64) & 0xFFFF) as u16;
    i.base_lo = e.as_usize() as u64 >> 16;
    i.gdt_selector = segmentation::cs().bits();

    use x86::shared::descriptor::*;
    use x86::shared::PrivilegeLevel::Ring0;

    i.flags = Flags::from_priv(Ring0)
                  .const_or(FLAGS_TYPE_SYS_NATIVE_INTERRUPT_GATE)
                  .const_or(FLAGS_PRESENT);
}
