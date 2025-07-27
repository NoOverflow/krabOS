use crate::libs::arch;

/*
    TODO: This is heavily biased for x86_64 architecture, will need to refactor with mappings between the IRQ index of various archs
          if we ever support other archs.
*/
pub fn handle_interrupt(context: &mut arch::internal::interrupts::ctx::Context) {
    return;
}
