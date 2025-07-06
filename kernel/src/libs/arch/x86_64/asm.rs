use super::gdt::GdtDescriptor;
use core::arch::asm;

#[inline]
pub unsafe fn cli() {
    unsafe {
        asm!("cli");
    }
}

#[inline]
pub unsafe fn outb(port: usize, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value, options(nostack));
    }
}

#[inline]
pub unsafe fn inb(port: usize) -> u8 {
    unsafe {
        let ret: u8;

        asm!("in al, dx", in("dx") port, out("al") ret , options(nostack));
        ret
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
