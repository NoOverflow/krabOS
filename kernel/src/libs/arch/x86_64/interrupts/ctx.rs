use core::fmt::Debug;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Registers {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rax: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Context {
    pub isr_index: u64,
    pub registers: Registers,
    pub error_code: u64,
    pub eip: u64,
    pub cs: u64,
    pub eflags: u64,
}

impl Debug for Registers {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            f,
            "RAX: {:0>16x}  RBX: {:0>16x}  RCX: {:0>16x}  RDX: {:0>16x}  RBP: {:0>16x}",
            self.rax, self.rbx, self.rcx, self.rdx, self.rbp
        )?;
        writeln!(
            f,
            "RDI: {:0>16x}  RSI: {:0>16x}  R8:  {:0>16x}  R9:  {:0>16x}  R10: {:0>16x}",
            self.rdi, self.rsi, self.r8, self.r9, self.r10
        )?;
        writeln!(
            f,
            "R11: {:016x}  R12: {:0>16x}  R13: {:0>16x}  R14: {:0>16x}  R15: {:0>16x}",
            self.r11, self.r12, self.r13, self.r14, self.r15
        )?;
        Ok(())
    }
}

impl Debug for Context {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            f,
            "EIP: {:0>16x}  CS:  {:0>16x}  EFLAGS: {:0>16x}",
            self.eip, self.cs, self.eflags
        )?;
        Ok(())
    }
}
