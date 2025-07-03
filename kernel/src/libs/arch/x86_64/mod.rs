pub mod asm;
pub mod gdt;
pub mod idt;
pub mod isr;

struct CpuContext {
    gdt: [u64; 5],
}

//static mut IDT: idt::Idt =

// NOTE: Yeah buddy you'll have to modify some of that for multi-proc support innit bruv
static mut CPU_CONTEXT: CpuContext = CpuContext {
    gdt: [0, 0, 0, 0, 0],
};

#[allow(static_mut_refs)]
pub fn init() {
    gdt::load(unsafe { &mut CPU_CONTEXT.gdt });
    // idt::load();
}
