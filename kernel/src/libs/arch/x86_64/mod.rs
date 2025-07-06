use crate::{info, libs::arch::x86_64::{
    gdt::{SegmentSelector, CPL_RING_0},
    idt::{Idt, IdtDescriptor, IdtGateDescriptor, IdtGateDescriptorProperties},
    isr::isr_handler,
}};
use core::arch::asm;
use crate::KERNEL_CONTEXT;
use core::fmt::Write;

pub mod asm;
pub mod gdt;
pub mod idt;
pub mod isr;

struct CpuContext {
    gdt: [u64; 5],
    idtr: Option<IdtDescriptor>,
}

//static mut IDT: idt::Idt =

// NOTE: Yeah buddy you'll have to modify some of that for multi-proc support innit bruv
static mut CPU_CONTEXT: CpuContext = CpuContext {
    gdt: [0, 0, 0, 0, 0],
    idtr: None,
};

#[allow(static_mut_refs)]
pub unsafe fn init() {
    gdt::load(unsafe { &mut CPU_CONTEXT.gdt });

    let idtr_default: IdtGateDescriptor = IdtGateDescriptor::new(
        isr_handler as _,
        SegmentSelector {
            global_descriptor_table: true,
            index: 1, // This will cause issues lmao
            requested_privilege: CPL_RING_0,
        },
        IdtGateDescriptorProperties {
            gate_type: idt::IdtGateType::Interrupt,
            privilege_level: CPL_RING_0,
        },
        0,
    );

    unsafe {
        CPU_CONTEXT.idtr = Some(IdtDescriptor { size: (size_of::<IdtGateDescriptor>() * 256) as u16 - 1, idt_offset: (&[idtr_default; 256]) as *const Idt});
        // Unmask PIC
        asm!(
            "mov al, 0x0",
            "out 0x21, al",
            "out 0xa1, al",
        );
        info!("PIC unmasked");
        idt::load(CPU_CONTEXT.idtr.as_ref().unwrap());
        asm!("sti");
    }
}
