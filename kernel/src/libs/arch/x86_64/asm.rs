use core::arch::asm;

#[inline]
pub unsafe fn cli() {
    unsafe {
        asm!("cli");
    }
}
