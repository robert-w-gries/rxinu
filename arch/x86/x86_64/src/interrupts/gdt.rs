use core::mem;

#[cfg(target_arch = "x86")]
use x86::bits32::task::TaskStateSegment;

#[cfg(target_arch = "x86_64")]
use x86::bits64::task::TaskStateSegment;

use x86::shared::PrivilegeLevel;
use x86::shared::dtables::{self, DescriptorTablePointer};
use x86::shared::segmentation::{self, SegmentDescriptor, SegmentSelector, Type, CODE_CONFORMING, DATA_WRITE};
use x86::shared::task::load_tr;

const GDT_NULL: usize = 0;
const GDT_KERNEL_CODE: usize = 1;
const GDT_KERNEL_DATA: usize = 2;
const GDT_TSS: usize = 3;

static mut GDTR: DescriptorTablePointer<SegmentDescriptor> = DescriptorTablePointer {
    limit: 0,
    base: 0 as *const SegmentDescriptor,
};

static mut GDT: [SegmentDescriptor; 4] = [
    // Null
    SegmentDescriptor::NULL,
    // Kernel code
    SegmentDescriptor::new(0, 0, Type::Code(CODE_CONFORMING), false, PrivilegeLevel::Ring0),
    // Kernel data
    SegmentDescriptor::new(0, 0, Type::Data(DATA_WRITE), false, PrivilegeLevel::Ring0),
    // TSS: set up in init
    SegmentDescriptor::NULL,
];

pub fn init(TSS: &'static TaskStateSegment) {
    GDTR.limit = (GDT.len() * mem::size_of::<SegmentDescriptor>() - 1) as u16;
    GDTR.base = GDT.as_ptr();

    // TODO: Investigate PrivilegeLevel for TSS
    let limit: u32 = mem::size_of::<TaskStateSegment>() as u32;
    GDT[GDT_TSS] = SegmentDescriptor::new(&TSS as *const _ as u32,
                                          limit,
                                          Type::Data(DATA_WRITE),
                                          false, // dirty flag
                                          PrivilegeLevel::Ring0);

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
