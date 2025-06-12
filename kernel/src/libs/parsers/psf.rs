// static FONT: &[u8]

#[derive(PartialEq, Eq, Debug)]
pub enum PsfVersion {
    Psf1,
    Psf2,
}

#[derive(PartialEq, Eq, Debug)]
pub struct PsfFontMode {
    pub mode_512: bool,
    pub mode_hastab: bool,
    pub mode_seq: bool,
}

impl PsfFontMode {
    pub fn from(raw: u8) -> Self {
        Self {
            mode_512: (raw & 0x01) != 0,
            mode_hastab: (raw & 0x02) != 0,
            mode_seq: (raw & 0x04) != 0,
        }
    }
}

pub struct Psf1 {
    pub mode: PsfFontMode,
}

pub struct Psf2 {
    // TODO: Implement PSF2 structure
}

pub struct PsfFont {
    pub version: PsfVersion,
    pub psf1: Option<Psf1>,
    pub psf2: Option<Psf2>,
    pub glyphs: &'static [u8],
    pub glyph_size: (u8, u8),
    pub glyph_stride: u8,
}

impl PsfFont {
    fn parse_psf1(font: &'static [u8]) -> Option<Self> {
        if font.len() < 4 {
            return None; // PSF1 header is at least 4 bytes
        }

        Some(Self {
            version: PsfVersion::Psf1,
            psf1: Some(Psf1 {
                mode: PsfFontMode::from(font[2]),
            }),
            psf2: None,
            glyphs: font.get(4..).unwrap(),
            glyph_size: (8, font[3]),
            // PSF1 glyphs are always 8 pixels wide, so 2 bytes per glyph (16 bits)
            glyph_stride: 1,
        })
    }

    pub fn parse(font: &'static [u8]) -> Option<Self> {
        let version = match font.get(0..4) {
            Some(&[0x36, 0x04, _, _]) => PsfVersion::Psf1,
            Some(&[0x72, 0xB5, 0x4A, 0x86]) => PsfVersion::Psf2,
            _ => return None,
        };

        match version {
            PsfVersion::Psf1 => {
                return Self::parse_psf1(font);
            }
            PsfVersion::Psf2 => {
                // TODO: Implement PSF2 parsing
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::Read;

    use crate::libs::parsers::psf::PsfFont;

    #[test]
    fn test_parse_psf1() {
        static FONT: &[u8] = include_bytes!("../../../resources/fonts/zap-light16.psf");

        let result = PsfFont::parse(FONT);

        assert!(result.is_some());
        let font = result.unwrap();

        assert_eq!(font.version, super::PsfVersion::Psf1);
        assert!(font.psf1.is_some());
        assert_eq!(font.glyph_size, (8, 16));
        assert_eq!(font.glyphs[0], 0x00);
        assert_eq!(font.glyphs[1], 0x00);
        assert_eq!(font.glyphs[2], 0x00);
        assert_eq!(font.glyphs[3], 0x3e);
    }
}
