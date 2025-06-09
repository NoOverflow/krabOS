use limine::{
    framebuffer::Framebuffer,
    request::{FramebufferRequest, RequestsEndMarker, RequestsStartMarker},
};

static FONT: &[u8] = include_bytes!("../../../resources/fonts/zap-light16.psf");

pub struct Vga<'a> {
    framebuffer: &'a Framebuffer<'a>,
}

impl<'a> Vga<'a> {
    pub fn init(&mut self) {
        let mut color: u32 = 0x000000;

        for i in 0..self.framebuffer.width() * self.framebuffer.height() {
            unsafe {
                self.framebuffer
                    .addr()
                    .cast::<u32>()
                    .add(i as usize)
                    .write(color | 0x000000FF);
            };
            color = color.wrapping_add(0x00010000);
        }
    }

    pub fn new(framebuffer: &'a Framebuffer<'a>) -> Self {
        Self { framebuffer }
    }
}
