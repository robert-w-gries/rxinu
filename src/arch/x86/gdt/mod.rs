use arch::x86::memory::MemoryController;
use core::mem;
use spin::Once;
use x86::current::segmentation::set_cs;
use x86::shared::PrivilegeLevel;
use x86::shared::dtables::{self, DescriptorTablePointer};
use x86::shared::segmentation::SegmentDescriptor;
use x86::shared::task::load_tr;

pub use self::current::{create_gdt, tss, GDT_SIZE};

// Segment Selector Index
const GDT_KERNEL_CODE: usize = 1;
const GDT_KERNEL_DATA: usize = 2;
#[allow(dead_code)]
const GDT_USER_CODE: usize = 3;
#[allow(dead_code)]
const GDT_USER_DATA: usize = 4;
const GDT_TSS: usize = 5;

pub type GdtArray = [SegmentDescriptor; GDT_SIZE];
pub static GDT: Once<GdtArray> = Once::new();

pub fn init(memory_controller: &mut MemoryController) {
    let tss = tss(memory_controller);
    let gdt: &GdtArray = create_gdt(&tss);

    let gdtr: DescriptorTablePointer<SegmentDescriptor> = DescriptorTablePointer {
        base: gdt.as_ptr(),
        limit: (gdt.len() * mem::size_of::<SegmentDescriptor>() - 1) as u16,
    };

    unsafe {
        use x86::shared::segmentation::*;

        dtables::lgdt(&gdtr);

        set_cs(SegmentSelector::new(
            GDT_KERNEL_CODE as u16,
            PrivilegeLevel::Ring0,
        ));
        load_ds(SegmentSelector::new(
            GDT_KERNEL_DATA as u16,
            PrivilegeLevel::Ring0,
        ));
        load_es(SegmentSelector::new(
            GDT_KERNEL_DATA as u16,
            PrivilegeLevel::Ring0,
        ));
        load_fs(SegmentSelector::new(
            GDT_KERNEL_DATA as u16,
            PrivilegeLevel::Ring0,
        ));
        load_gs(SegmentSelector::new(
            GDT_KERNEL_DATA as u16,
            PrivilegeLevel::Ring0,
        ));
        load_ss(SegmentSelector::new(
            GDT_KERNEL_DATA as u16,
            PrivilegeLevel::Ring0,
        ));

        load_tr(SegmentSelector::new(GDT_TSS as u16, PrivilegeLevel::Ring3));
    }
}

#[cfg(target_arch = "x86")]
mod bits32;
#[cfg(target_arch = "x86_64")]
mod bits64;

mod current {
    #[cfg(target_arch = "x86")]
    pub use super::bits32::*;
    #[cfg(target_arch = "x86_64")]
    pub use super::bits64::*;
}
