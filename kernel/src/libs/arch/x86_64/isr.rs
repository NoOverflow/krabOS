use core::arch::naked_asm;

use crate::libs::arch::x86_64::CPU_CONTEXT;

pub fn generic_handler() {
    let gdt = unsafe { &CPU_CONTEXT.gdt };
}

#[naked]
pub unsafe fn isr_handler() {
    unsafe {
        naked_asm!("pushad", "cld", "popad", "iretq");
    }
}
