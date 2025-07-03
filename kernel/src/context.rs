use limine::framebuffer::Framebuffer;

use crate::libs::{drivers::logs::sinks::vga::Vga, generic::logging::logger::Logger};

#[derive(Default)]
pub struct KernelContext<'a> {
    pub framebuffer: Option<Framebuffer<'a>>,
    // I hate myself for this.
    pub vga: Option<Vga<'a>>,
    pub logger: Option<Logger<'a>>,
}
