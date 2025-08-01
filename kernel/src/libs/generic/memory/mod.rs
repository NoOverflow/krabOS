use crate::_log;
use crate::info;
use crate::libs::generic::memory::pfa::PageFrameAllocator;
use limine::{memory_map::EntryType, response::MemoryMapResponse};

mod pfa;

pub fn init(mmap: Option<&'static MemoryMapResponse>) {
    assert!(mmap.is_some());
    let entries: &[&limine::memory_map::Entry] = mmap.unwrap().entries();

    info!("Memory map detection:");
    for entry in entries {
        if entry.entry_type == EntryType::RESERVED {
            continue;
        }
        _log!(
            "",
            "        [{:#x} - {:#x}] {} ({}MB)",
            entry.base,
            entry.base + entry.length,
            match entry.entry_type {
                EntryType::USABLE => "Free memory",
                EntryType::FRAMEBUFFER => "VESA Framebuffer",
                EntryType::EXECUTABLE_AND_MODULES => "Current kernel",
                EntryType::ACPI_NVS => "Reserved ACPI",
                EntryType::ACPI_RECLAIMABLE => "Reclaimable ACPI",
                EntryType::BAD_MEMORY => "Unusable memory (Bad or corrupted memory)",
                _ => "Unknown",
            },
            entry.length / 1024 / 1024
        );
    }

    // TODO: Dynamic page frame size
    let mut pfa = PageFrameAllocator::new(entries, 4096);

    info!("Usable memory detected {}MiB", pfa.size() / 1024 / 1024);
    for i in 0..2048 {
        let add = pfa.alloc();

        if i > 2045 {
            info!("PFA gave us 0x{:02x}", add);
        }
    }
}
