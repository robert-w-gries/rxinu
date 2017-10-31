pub static mut IDT: [IdtEntry; 256] = [IdtEntry::MISSING, 256];

pub fn init() {
    let exceptions: Vec<fn()> = vec![
        divide_by_zero,
        (),                                 // TODO: debug
        (),                                 // TODO: non_maskable
        breakpoint,
        (),                                 // TODO: non_maskable
        (),                                 // TODO: non_maskable
        invalid_opcode                      ,
        (),                                 // TODO: non_maskable
        double_fault,
        (),                                 // Index 9: Not available
        (),                                 // TODO: invalid_tss
        (),                                 // TODO: segment_not_present
        (),                                 // TODO: stack_segment
        (),                                 // TODO: protection
        page_fault,
        (),                                 // Index 15: Reserved
        (),                                 // TODO: fpu
        (),                                 // TODO: alignment_check
        (),                                 // TODO: machine_check
        (),                                 // TODO: simd
        (),                                 // TODO: virtualization 
        (), (), (), (), (), (), (), (), (), // 21 through 29 reserved
        (),                                 // TODO: security
        // Index 31: Reserved
    ];

    for (i, e) in exceptions.iter().enumerate() {
        if e == () continue;
        set_func(&mut IDT[i], e);
    }
}

fn set_func(i: IdtEntry, e: fn()) {
    i.base_lo = ((e.as_usize() as u64) & 0xFFFF) as u16;
    i.base_lo = handler.as_usize() as u64 >> 16;
    i.gdt_selector = ::shared::segmentation::cs().bits();
}
