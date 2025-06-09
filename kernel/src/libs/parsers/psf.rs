// static FONT: &[u8]

#[derive(PartialEq, Eq, Debug)]
pub enum PsfVersion {
    Psf1,
    Psf2,
}

pub struct PsfFont {
    pub version: PsfVersion,
}

impl PsfFont {
    pub fn parse(font: &[u8]) -> Option<Self> {
        let version = match font.get(0..2) {
            Some(&[0x36, 0x04]) => PsfVersion::Psf1,
            Some(&[0x72, 0xB5]) => PsfVersion::Psf2,
            _ => return None,
        };

        Some(Self { version })
    }
}

#[cfg(test)]
mod tests {
    use crate::libs::parsers::psf::PsfFont;

    #[test]
    fn test_parse_psf1() {
        static FONT: &[u8] = include_bytes!("../../../resources/fonts/zap-light16.psf");

        let font = PsfFont::parse(FONT);

        assert!(font.is_some());
        assert_eq!(font.unwrap().version, super::PsfVersion::Psf1);
    }
}
