pub mod asm;
pub mod gdt;
pub mod idt;

// TODO: Move to per-proc
static mut GDT: [u64; 5] = [0, 0, 0, 0, 0];

#[allow(static_mut_refs)]
pub fn init() {
    gdt::load(unsafe { &mut GDT });
    idt::load();
}
