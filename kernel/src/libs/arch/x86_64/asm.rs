use super::gdt::GdtDescriptor;
use core::arch::asm;

#[inline]
pub unsafe fn cli() {
    unsafe {
        asm!("cli");
    }
}

#[inline]
pub unsafe fn load_gdt(gdtr: &GdtDescriptor) {
    unsafe {
        asm!(
            "cli",
            "lgdt [{gdtr}]",
            "push {codeseg_offset}",
            "lea RAX, [rip + 2f]",
            "push RAX",
            "retfq",
        "2:",
            "mov ax, {dataseg_offset}",
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            gdtr = in(reg) gdtr,
            // Ideally you would get the offsets dynamically from the Gdtr, but we convert to u64 early
            // making it impossible.
            codeseg_offset = const 0x8,
            dataseg_offset = const 0x10,
            lateout("rax") _
        );
    }
}
