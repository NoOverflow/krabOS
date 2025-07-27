use seq_macro::seq;

use crate::libs::arch::x86_64::cpu::CpuInfo;
use crate::{
    info,
    libs::arch::x86_64::{
        gdt::{CPL_RING_0, SegmentSelector},
        interrupts::idt::{Idt, IdtDescriptor, IdtGateDescriptor, IdtGateDescriptorProperties},
    },
};
use core::arch::asm;

pub mod asm;
pub mod cpu;
pub mod gdt;
pub mod registers;
pub mod interrupts {
    pub mod ctx;
    pub mod idt;
    pub mod isr;
}

struct CpuContext {
    gdt: [u64; 5],
    idtr: Option<IdtDescriptor>,
    info: Option<CpuInfo>,
}

// NOTE: Yeah buddy you'll have to modify some of that for multi-proc support innit bruv
static mut CPU_CONTEXT: CpuContext = CpuContext {
    gdt: [0, 0, 0, 0, 0],
    idtr: None,
    info: None,
};

#[allow(static_mut_refs)]
pub unsafe fn init() {
    let mut idtr: [IdtGateDescriptor; 256] = [Default::default(); 256];

    gdt::load(unsafe { &mut CPU_CONTEXT.gdt });

    seq!(N in 0..256 {
        let igtgd: IdtGateDescriptor = IdtGateDescriptor::new(
            crate::arch::internal::interrupts::isr::isr_handler~N as _,
            SegmentSelector {
                local_descriptor_table: false,
                index: 1, // This will cause issues lmao
                requested_privilege: CPL_RING_0,
            },
            IdtGateDescriptorProperties {
                gate_type: interrupts::idt::IdtGateType::Interrupt,
                privilege_level: CPL_RING_0,
            },
            0,
        );

        idtr[N] = igtgd;
    });

    unsafe {
        CPU_CONTEXT.idtr = Some(IdtDescriptor {
            size: (size_of::<IdtGateDescriptor>() * 256) as u16 - 1,
            idt_offset: (&idtr) as *const Idt,
        });
        CPU_CONTEXT.info = Some(CpuInfo::new());
        CPU_CONTEXT
            .info
            .as_mut()
            .unwrap()
            .request(cpu::CpuIdRequest::BasicFeatures);
        info!(
            "APIC supported: {}",
            CPU_CONTEXT
                .info
                .as_ref()
                .unwrap()
                .basic_features
                .as_ref()
                .unwrap()
                .flags
                .contains(cpu::BasicFeaturesFlags::APIC)
        );
        // Unmask PIC
        asm!("mov al, 0x1", "out 0x21, al", "out 0xa1, al",);
        info!("PIC unmasked");
        interrupts::idt::load(CPU_CONTEXT.idtr.as_ref().unwrap());
        asm!("sti");
    }
}
