use limine::framebuffer::Framebuffer;

use crate::libs::{drivers::logs::sinks::Sink, parsers::psf::PsfFont};

static FONT_DATA: &[u8] = include_bytes!("../../../../../resources/fonts/zap-light16.psf");

pub struct Vga<'a> {
    framebuffer: &'a Framebuffer<'a>,
    font: PsfFont,
    cursor_pos: (u32, u32),
}

impl<'a> Vga<'a> {
    pub fn init(&mut self) {}

    pub fn clear(framebuffer: &'a Framebuffer<'a>, color: u32) {
        for i in (framebuffer.width() * 50)..framebuffer.width() * framebuffer.height() {
            unsafe {
                framebuffer
                    .addr()
                    .cast::<u32>()
                    .add(i as usize)
                    .write(color);
            };
        }
    }

    pub fn new(framebuffer: &'a Framebuffer<'a>) -> Self {
        let font = PsfFont::parse(FONT_DATA);

        if font.is_none() {
            Vga::clear(framebuffer, 0xFF0000FF);
            panic!("Failed to parse font data");
        }
        Self {
            framebuffer,
            font: font.unwrap(),
            cursor_pos: (0, 0),
        }
    }
}

impl<'a> Sink for Vga<'a> {
    fn putchar(&mut self, c: char) {
        if c == '\n' {
            self.cursor_pos.0 = 0;
            self.cursor_pos.1 += 1;
            return;
        }

        let glyph_start: usize =
            (c as usize) * (self.font.glyph_stride as usize * self.font.glyph_size.1 as usize);
        let glyph_data: &[u8] = &self.font.glyphs[glyph_start..];

        for y in 0..self.font.glyph_size.1 {
            for x in 0..self.font.glyph_size.0 {
                let pixel_color: u32 = if glyph_data
                    [y as usize * self.font.glyph_stride as usize + (x as usize / 8)]
                    & (1 << 7 - (x % 8))
                    != 0
                {
                    0xFFFFFFFF // White pixel
                } else {
                    0x00000000 // Transparent pixel
                };

                let absolute_x: u64 =
                    (self.cursor_pos.0 as u64 * self.font.glyph_size.0 as u64) + x as u64;
                let absolute_y: u64 =
                    (self.cursor_pos.1 as u64 * self.font.glyph_size.1 as u64) + y as u64;

                unsafe {
                    self.framebuffer
                        .addr()
                        .cast::<u32>()
                        .add((absolute_y * self.framebuffer.width() + absolute_x) as usize)
                        .write(pixel_color);
                };
            }
        }
        self.cursor_pos.0 += 1;
        if self.cursor_pos.0 as u64 * self.font.glyph_size.0 as u64 >= self.framebuffer.width() {
            self.cursor_pos.0 = 0;
            self.cursor_pos.1 += 1;
        }
    }

    fn putstr(&mut self, s: &str) {
        for c in s.chars() {
            self.putchar(c);
        }
    }
}
