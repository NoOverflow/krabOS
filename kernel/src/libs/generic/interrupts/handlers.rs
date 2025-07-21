use crate::libs::arch;

/*
    TODO: This is heavily biased for x86_64 architecture, will need to refactor with mappings between the IRQ index of various archs
          if we ever support other archs.
*/
pub fn handle_interrupt(context: &mut arch::internal::interrupts::ctx::Context) {
    if context.isr_index == 0x1 {
        return;
    }
    panic!(
        "An unhandled CPU interrupt occured, {} (error code: {:x})\n\n{:?}{:?}",
        match context.isr_index {
            0x0 => "division by zero",
            0x1 => "debug instruction",
            0x2 => "NMI interrupt",
            0x3 => "breakpoint",
            0xE => "page fault",
            _ => "unknown error",
        },
        context.error_code,
        context,
        context.registers
    );
}
