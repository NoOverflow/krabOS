use self::colors::Color;

pub mod colors;

const VGA_LINES: usize = 25;
const VGA_COLUMNS: usize = 80;

#[repr(C)]
struct Character {
    character: u8,
    attribute: u8,
}

#[repr(transparent)]
struct Buffer {
    chars: [[Character; VGA_COLUMNS]; VGA_LINES],
}

pub struct VGATty {
    pub x: usize,
    pub y: usize,

    vga_buffer: &'static mut Buffer,
    char_attribute: u8,
}

impl VGATty {
    pub fn bind() -> VGATty {
        VGATty {
            x: 0,
            y: 0,
            char_attribute: 0x0F,
            vga_buffer: unsafe { &mut *(0xB8000 as *mut Buffer) },
        }
    }

    pub fn clear_screen(&mut self) {
        // TODO: Replace with a memset
        for y in 0..25 {
            for x in 0..80 {
                self.vga_buffer.chars[y][x] = Character {
                    character: b' ',
                    attribute: self.char_attribute,
                };
            }
        }
    }

    pub fn set_foreground_color(&mut self, color: Color) {
        self.char_attribute |= color as u8;
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.char_attribute |= (color as u8) << 4;
    }

    pub fn write(&mut self, str: &[u8]) {
        for i in 0..str.len() {
            match str[i] {
                b'\n' => {
                    self.x = 0;
                    self.y += 1;
                }
                c => {
                    self.vga_buffer.chars[self.y][self.x] = Character {
                        character: c,
                        attribute: self.char_attribute,
                    };
                    self.x += 1;
                    if self.x > VGA_COLUMNS {
                        self.x = 0;
                        self.y += 1;
                    }
                }
            }
        }
    }
}
