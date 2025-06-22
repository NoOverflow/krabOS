use crate::libs::arch::x86_64::asm::load_gdt;
use bitflags::bitflags;

const CPL_RING_3: u8 = 0b11; // Usermode CPU privilege level
const CPL_RING_0: u8 = 0b00; // Kernel CPU privilege level

bitflags! {
    struct GdtAccessByte: u8 {
        const Accessed = 1;
        const ReadWrite = 1 << 1;
        const DirectionConforming = 1 << 2;
        const Executable = 1 << 3;
        const DescriptorType = 1 << 4;
        const UserModePrivilege = CPL_RING_3 << 5;
        const KernelPrivilege = CPL_RING_0 << 5;
        const Present = 1 << 7;
    }

    struct GdtFlag: u8 {
        const LongMode = 1 << 1;
        const Size = 1 << 2;
        const Granularity = 1 << 3;
    }
}

struct GdtSegmentDescriptor {
    pub limit: u32,
    pub base: u32,
    pub flags: GdtFlag,
    pub access: GdtAccessByte,
}

#[repr(C, packed)]
pub struct GdtDescriptor {
    pub size: u16,
    pub gdt: *const u64,
}

impl Into<u64> for GdtSegmentDescriptor {
    fn into(self) -> u64 {
        let mut ret: u64 = 0;

        ret |= (self.base as u64 & 0xFF000000) << 32;
        ret |= (self.flags.bits() as u64 & 0xF) << 52;
        ret |= (self.limit as u64 & 0xF0000) << 32;
        ret |= (self.access.bits() as u64 & 0xFF) << 40;
        ret |= (self.base as u64 & 0xFF0000) << 16;
        ret |= (self.base as u64 & 0x00FFFF) << 16;
        ret |= self.limit as u64 & 0xFFFF;

        ret
    }
}

pub fn load() {
    let gdt: &[u64] = &[
        // Null descriptor
        GdtSegmentDescriptor {
            base: 0,
            limit: 0,
            access: GdtAccessByte::empty(),
            flags: GdtFlag::empty(),
        }
        .into(),
        // Kernel mode code segment
        GdtSegmentDescriptor {
            base: 0,
            limit: 0xFFFFF,
            access: GdtAccessByte::Present
                | GdtAccessByte::KernelPrivilege
                | GdtAccessByte::DescriptorType
                | GdtAccessByte::Executable
                | GdtAccessByte::Accessed,
            flags: GdtFlag::Granularity | GdtFlag::LongMode,
        }
        .into(),
        // Kernel mode data segment
        GdtSegmentDescriptor {
            base: 0,
            limit: 0xFFFFF,
            access: GdtAccessByte::Present
                | GdtAccessByte::KernelPrivilege
                | GdtAccessByte::DescriptorType
                | GdtAccessByte::ReadWrite
                | GdtAccessByte::Accessed,
            flags: GdtFlag::Granularity | GdtFlag::Size,
        }
        .into(),
        // User mode code segment
        GdtSegmentDescriptor {
            base: 0,
            limit: 0xFFFFF,
            access: GdtAccessByte::Present
                | GdtAccessByte::UserModePrivilege
                | GdtAccessByte::DescriptorType
                | GdtAccessByte::Executable
                | GdtAccessByte::Accessed,
            flags: GdtFlag::Granularity | GdtFlag::LongMode,
        }
        .into(),
        // User mode data segment
        GdtSegmentDescriptor {
            base: 0,
            limit: 0xFFFFF,
            access: GdtAccessByte::Present
                | GdtAccessByte::UserModePrivilege
                | GdtAccessByte::DescriptorType
                | GdtAccessByte::ReadWrite
                | GdtAccessByte::Accessed,
            flags: GdtFlag::Granularity | GdtFlag::Size,
        }
        .into(),
        // TODO: Add TSS
    ];
    let gdtr = GdtDescriptor {
        gdt: gdt.as_ptr(),
        size: (gdt.len() * size_of::<u64>() - 1) as u16,
    };

    unsafe {
        load_gdt(&gdtr);
    }
}

#[cfg(test)]
mod tests {
    use crate::libs::arch::x86_64::gdt::GdtAccessByte;
    use crate::libs::arch::x86_64::gdt::GdtDescriptor;
    use crate::libs::arch::x86_64::gdt::GdtFlag;
    use crate::libs::arch::x86_64::gdt::GdtSegmentDescriptor;

    #[test]
    fn test_serialize_gdtsegdesc() {
        let segdesc = GdtSegmentDescriptor {
            limit: 0xA5277,
            base: 0xDF627451,
            access: GdtAccessByte::Accessed
                | GdtAccessByte::KernelPrivilege
                | GdtAccessByte::ReadWrite
                | GdtAccessByte::Present,
            flags: GdtFlag::LongMode | GdtFlag::Granularity,
        };
        let result: u64 = segdesc.into();

        // Expected:
        // base_up[0b1101_1111] flags[0b1010] limit_up[0b1010]  ab[0b10000011] base_up[0b0110_0010]
        // base_low[0b0111_0100_0101_0001] limit_low[0b0101001001110111]
        // == 0xDFAA836274515277
        assert_eq!(result, 0xDFAA836274515277);
    }

    #[test]
    fn test_struct_sizes() {
        assert_eq!(size_of::<GdtDescriptor>() * 8, 80);
    }
}
