use crate::KERNEL_CONTEXT;
use crate::libs::arch::x86_64::asm::{CpuIdRegisterOrder, cpuid};
use crate::warning;
use bitflags::bitflags;
use core::fmt::Write;

#[derive(Debug)]
pub enum CpuIdRequest {
    BasicFeatures = 0x01,
    ExtendedFeatures = 0x07,
}

bitflags! {
    #[derive(Default)]
    pub struct BasicFeaturesFlags: u64 {
        /* EDX */
        const FPU = 1;
        const VME = 1 << 1;
        const DE = 1 << 2;
        const PSE = 1 << 3;
        const TSC = 1 << 4;
        const MSR = 1 << 5;
        const PAE = 1 << 6;
        const MCE = 1 << 7;
        const CX8 = 1 << 8;
        const APIC = 1 << 9;
        const SEP = 1 << 11;
        const MTRR = 1 << 12;
        const PGE = 1 << 13;
        const MCA = 1 << 14;
        const CMOV = 1 << 15;
        const PAT = 1 << 16;
        const PSE36 = 1 << 17;
        const PSN = 1 << 18;
        const CLFLUSH = 1 << 19;
        const NX = 1 << 20;
        const DS = 1 << 21;
        const ACPI = 1 << 22;
        const MMX = 1 << 23;
        const FXSR = 1 << 24;
        const SSE = 1 << 25;
        const SSE2 = 1 << 26;
        const SS = 1 << 27;
        const HTT = 1 << 28;
        const TM = 1 << 29;
        const IA64 = 1 << 30;
        const PBE = 1 << 31;

        /* ECX */
        const SSE3 = 1 << 32;
        const PCLMUL = 1 << 33;
        const DTES64 = 1 << 34;
        const MONITOR = 1 << 35;
        const DS_CPL = 1 << 36;
        const VMX = 1 << 37;
        const SMX = 1 << 38;
        const EST = 1 << 39;
        const TM2 = 1 << 40;
        const SSSE3 = 1 << 41;
        const CID = 1 << 42;
        const SDBG = 1 << 43;
        const FMA = 1 << 44;
        const CX16 = 1 << 45;
        const XTPR = 1 << 46;
        const PDCM = 1 << 47;
        const PCID = 1 << 49;
        const DCA = 1 << 50;
        const SSE4_1 = 1 << 51;
        const SSE4_2 = 1 << 52;
        const X2APIC = 1 << 53;
        const MOVBE = 1 << 54;
        const POPCNT = 1 << 55;
        const AES = 1 << 57;
        const XSAVE = 1 << 56;
        const OSXSAVE = 1 << 59;
        const AVX = 1 << 60;
        const F16C = 1 << 61;
        const RDRAND = 1 << 62;
        const HYPERVISOR = 1 << 63;
    }
}

#[derive(Default)]
pub struct BasicFeatures {
    pub flags: BasicFeaturesFlags,
}

#[derive(Default)]
pub struct CpuInfo {
    pub basic_features: Option<BasicFeatures>,
}

impl CpuInfo {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub unsafe fn request(&mut self, request: CpuIdRequest) {
        // TODO: Check if CPUID is supported
        let request_result: [u32; 4] = unsafe { cpuid(CpuIdRequest::BasicFeatures.into()) };

        match request {
            CpuIdRequest::BasicFeatures => {
                self.basic_features = Some(BasicFeatures {
                    flags: BasicFeaturesFlags::from_bits_truncate(
                        request_result[CpuIdRegisterOrder::EDX as usize] as u64
                            | ((request_result[CpuIdRegisterOrder::ECX as usize] as u64) << 32),
                    ),
                });
            }
            _ => {
                warning!("Unsupported CPU ID request: {:?}", request);
            }
        }
    }
}
