use core::arch::naked_asm;
use crate::libs::arch::x86_64::asm::outb;

#[unsafe(no_mangle)]
pub extern "C" fn generic_handler() {
    unsafe {
        outb(0x20, 0x20);
    }
}

// Note: There is no predefined macro to push all 15 general purpose registers (excluding RSP)
//       in x86_64 like PUSHAD so we make our own
//       to optimise slightly we decrement the RSP first and move each reg manually.
macro_rules! push_gpregs {
    () => {
        concat!(
            "sub rsp, 120;", // 15 * 8 bits
            "mov [rsp+112], rax;",
            "mov [rsp+104], rbx;",
            "mov [rsp+96], rcx;",
            "mov [rsp+88], rdx;",
            "mov [rsp+80], rbp;",
            "mov [rsp+72], rdi;",
            "mov [rsp+64], rsi;",
            "mov [rsp+56], r8;",
            "mov [rsp+48], r9;",
            "mov [rsp+40], r10;",
            "mov [rsp+32], r11;",
            "mov [rsp+24], r12;",
            "mov [rsp+16], r13;",
            "mov [rsp+8], r14;",
            "mov [rsp], r15;"
        )
    };
}

macro_rules! pop_gpregs {
    () => {
        concat!(
            "mov r15, [rsp];",
            "mov r14, [rsp+8];",
            "mov r13, [rsp+16];",
            "mov r12, [rsp+24];",
            "mov r11, [rsp+32];",
            "mov r10, [rsp+40];",
            "mov r9, [rsp+48];",
            "mov r8, [rsp+56];",
            "mov rsi, [rsp+64];",
            "mov rdi, [rsp+72];",
            "mov rbp, [rsp+80];",
            "mov rdx, [rsp+88];",
            "mov rcx, [rsp+96];",
            "mov rbx, [rsp+104];",
            "mov rax, [rsp+112];",
            "add rsp, 120;"
        )
    };
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn isr_handler() {
    naked_asm!(push_gpregs!(), "cld", "call {}", pop_gpregs!(), "iretq", sym generic_handler);
}
