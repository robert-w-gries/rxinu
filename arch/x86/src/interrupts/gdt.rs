use core::mem;

#[cfg(target_arch = "x86")] use x86::bits32::task::TaskStateSegment;
#[cfg(target_arch = "x86")] use x86::bits32::segmentation::{self, SegmentDescriptor, SegmentSelector};

#[cfg(target_arch = "x86_64")] use x86::bits64::task::TaskStateSegment;
#[cfg(target_arch = "x86_64")] use x86::bits64::segmentation::{self, SegmentBitness, SegmentDescriptor, SegmentSelector};

use x86::shared::PrivilegeLevel;
use x86::shared::dtables::{self, DescriptorTablePointer};
use x86::shared::segmentation::{Type, CODE_READ, DATA_WRITE};
use x86::shared::task::load_tr;

#[cfg(target_arch = "x86")] const GDT_SIZE: usize = 4;
#[cfg(target_arch = "x86_64")] const GDT_SIZE: usize = 5;

// Segment Selector Index
const GDT_KERNEL_CODE: usize = 1;
const GDT_KERNEL_DATA: usize = 2;
const GDT_TSS: usize = 3;

static mut GDT: [SegmentDescriptor; GDT_SIZE] = [SegmentDescriptor::NULL; GDT_SIZE];

static mut GDTR: DescriptorTablePointer<SegmentDescriptor> = DescriptorTablePointer {
    limit: 0,
    base: 0 as * const _,
};

#[cfg(target_arch = "x86")]
pub unsafe fn init(tss: &TaskStateSegment) {
    // TODO: Investigate PrivilegeLevel for TSS
    GDT[GDT_KERNEL_CODE] = SegmentDescriptor::new_memory(0,
        0,
        Type::Code(CODE_READ),
        false,
        PrivilegeLevel::Ring0);

    GDT[GDT_KERNEL_DATA] = SegmentDescriptor::new_memory(0,
        0,
        Type::Data(DATA_WRITE),
        false,
        PrivilegeLevel::Ring0);

    GDT[GDT_TSS] = SegmentDescriptor::new_tss(&tss, PrivilegeLevel::Ring0); 

    GDTR.base = GDT.as_ptr();
    GDTR.limit = (GDT.len() * mem::size_of::<SegmentDescriptor>() - 1) as u16;

    dtables::lgdt(&GDTR);

    // TODO: Investigate PrivilegeLevel for segment selectors and TSS load
    segmentation::set_cs(SegmentSelector::new(GDT_KERNEL_CODE as u16, PrivilegeLevel::Ring0));
    segmentation::load_ds(SegmentSelector::new(GDT_KERNEL_DATA as u16, PrivilegeLevel::Ring0));
    segmentation::load_es(SegmentSelector::new(GDT_KERNEL_DATA as u16, PrivilegeLevel::Ring0));
    segmentation::load_fs(SegmentSelector::new(GDT_KERNEL_DATA as u16, PrivilegeLevel::Ring0));
    segmentation::load_gs(SegmentSelector::new(GDT_KERNEL_DATA as u16, PrivilegeLevel::Ring0));
    segmentation::load_ss(SegmentSelector::new(GDT_KERNEL_DATA as u16, PrivilegeLevel::Ring0));

    load_tr(SegmentSelector::new(GDT_TSS as u16, PrivilegeLevel::Ring0));
}

#[cfg(target_arch = "x86_64")]
pub unsafe fn init(tss: &TaskStateSegment) {
    // TODO: Investigate PrivilegeLevel for TSS
    // Kernel code
    GDT[GDT_KERNEL_CODE] = SegmentDescriptor::new_memory(0,
        0,
        Type::Code(CODE_READ),
        false,
        PrivilegeLevel::Ring0,
        SegmentBitness::Bits64);

    // Kernel data
    GDT[GDT_KERNEL_DATA] = SegmentDescriptor::new_memory(0,
        0,
        Type::Data(DATA_WRITE),
        false,
        PrivilegeLevel::Ring0,
        SegmentBitness::Bits64);

    let tss_segs = SegmentDescriptor::new_tss(&tss, PrivilegeLevel::Ring0); 

    // TSS
    GDT[GDT_TSS] = tss_segs[0];
    GDT[GDT_TSS+1] = tss_segs[1];

    GDTR.base = GDT.as_ptr();
    GDTR.limit = (GDT.len() * mem::size_of::<SegmentDescriptor>() - 1) as u16;

    dtables::lgdt(&GDTR);

    // TODO: Investigate PrivilegeLevel for segment selectors and TSS load
    segmentation::set_cs(SegmentSelector::new(GDT_KERNEL_CODE as u16, PrivilegeLevel::Ring0));
    segmentation::load_ds(SegmentSelector::new(GDT_KERNEL_DATA as u16, PrivilegeLevel::Ring0));
    segmentation::load_es(SegmentSelector::new(GDT_KERNEL_DATA as u16, PrivilegeLevel::Ring0));
    segmentation::load_fs(SegmentSelector::new(GDT_KERNEL_DATA as u16, PrivilegeLevel::Ring0));
    segmentation::load_gs(SegmentSelector::new(GDT_KERNEL_DATA as u16, PrivilegeLevel::Ring0));
    segmentation::load_ss(SegmentSelector::new(GDT_KERNEL_DATA as u16, PrivilegeLevel::Ring0));

    load_tr(SegmentSelector::new(GDT_TSS as u16, PrivilegeLevel::Ring0));
}
