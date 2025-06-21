use asm::cli;

pub mod asm;
pub mod gdt;

pub fn init() {
    unsafe { cli() };
    gdt::load();
}
