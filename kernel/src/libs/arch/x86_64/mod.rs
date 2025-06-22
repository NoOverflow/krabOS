pub mod asm;
pub mod gdt;

pub fn init() {
    gdt::load();
}
