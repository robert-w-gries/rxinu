use arch::x86::memory::MemoryController;
use core::mem;
use spin::Once;
use x86::shared::PrivilegeLevel;
use x86::shared::dtables::{self, DescriptorTablePointer};
use x86::shared::segmentation::SegmentDescriptor;

pub use self::current::{create_gdt, tss, GDT_SIZE};

// Segment Selector Index
pub const GDT_KERNEL_CODE: usize = 1;
pub const GDT_KERNEL_DATA: usize = 2;
pub const GDT_USER_CODE: usize = 3;
pub const GDT_USER_DATA: usize = 4;
pub const GDT_TSS: usize = 5;

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
        use x86::current::segmentation::set_cs;
        use x86::shared::segmentation::{load_ss, SegmentSelector};
        use x86::shared::task::load_tr;

        dtables::lgdt(&gdtr);

        set_cs(SegmentSelector::new(
            GDT_KERNEL_CODE as u16,
            PrivilegeLevel::Ring0,
        ));

        load_selectors(GDT_KERNEL_DATA, PrivilegeLevel::Ring0);
        load_ss(SegmentSelector::new(
            GDT_KERNEL_DATA as u16,
            PrivilegeLevel::Ring0,
        ));

        load_tr(SegmentSelector::new(GDT_TSS as u16, PrivilegeLevel::Ring3));
    }
}

/// Load all selectors except for Stack Segment
/// Stack Segment cannot be loaded as PrivilegeLevel3 and it is not usually loaded anyway
/// [unsafe]
/// This function is purely assembly and is inherently unsafe
pub unsafe fn load_selectors(selector_index: usize, privilege: PrivilegeLevel) {
    use x86::shared::segmentation::{load_ds, load_es, load_fs, load_gs, SegmentSelector};

    load_ds(SegmentSelector::new(selector_index as u16, privilege));
    load_es(SegmentSelector::new(selector_index as u16, privilege));
    load_fs(SegmentSelector::new(selector_index as u16, privilege));
    load_gs(SegmentSelector::new(selector_index as u16, privilege));
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
