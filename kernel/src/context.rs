use core::time::Duration;

use limine::framebuffer::Framebuffer;

use crate::libs::{drivers::logs::sinks::vga::Vga, generic::logging::logger::Logger};

#[derive(Default)]
pub struct BootInfo {
    pub limine_base_revision: Option<u64>,
    pub kernel_phys_address: u64,
    pub kernel_virt_address: u64,
    pub hhdm: u64,
    pub rtc_boot: Option<Duration>,
}

#[derive(Default)]
pub struct KernelContext<'a> {
    pub framebuffer: Option<Framebuffer<'a>>,
    // I hate myself for this.
    pub vga: Option<Vga<'a>>,
    pub logger: Option<Logger<'a>>,
}
