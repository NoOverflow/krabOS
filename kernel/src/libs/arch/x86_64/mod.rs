use crate::libs::arch::x86_64::{
    gdt::{CPL_RING_0, SegmentSelector},
    idt::{Idt, IdtGateDescriptor, IdtGateDescriptorProperties},
    isr::isr_handler,
};

pub mod asm;
pub mod gdt;
pub mod idt;
pub mod isr;

struct CpuContext {
    gdt: [u64; 5],
    idt: Idt,
}

//static mut IDT: idt::Idt =

// NOTE: Yeah buddy you'll have to modify some of that for multi-proc support innit bruv
static mut CPU_CONTEXT: CpuContext = CpuContext {
    gdt: [0, 0, 0, 0, 0],
    idt: [],
};

#[allow(static_mut_refs)]
pub fn init() {
    gdt::load(unsafe { &mut CPU_CONTEXT.gdt });

    let idtr_default: IdtGateDescriptor = IdtGateDescriptor::new(
        isr_handler as _,
        SegmentSelector {
            global_descriptor_table: true,
            index: 1,
            requested_privilege: CPL_RING_0,
        },
        IdtGateDescriptorProperties {
            gate_type: idt::IdtGateType::Interrupt,
            privilege_level: CPL_RING_0,
        },
        0,
    );
    <
    idt::load();
}
