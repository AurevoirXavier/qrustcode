#[derive(Debug)]
pub enum Mode {
    Unknown,
    Numeric,
    Alphanumeric,
    Byte,
    Kanji,
    Chinese,
//    ECI,  TODO
}

impl Mode {
    pub fn to_usize(&self) -> usize {
        match self {
            Mode::Numeric => 0,
            Mode::Alphanumeric => 1,
            Mode::Byte => 2,
            Mode::Kanji => 3,
            Mode::Chinese => 4,
            _ => panic!(),
        }
    }

    pub fn not_support(&self, c: char) -> bool {
        match self {
            Mode::Numeric => match c {
                '0'...'9' => false,
                _ => true
            }
            Mode::Alphanumeric => if super::qrcode_info::alphanumeric_table(c as u8) == 0 { true } else { false }
            Mode::Byte => match c as u8 {
                0...255 => true,
                _ => false
            }
            Mode::Kanji => c < '\u{0800}' || c > '\u{4e00}',   // TODO range might not accurate
            Mode::Chinese => c < '\u{4e00}' || c > '\u{9fa5}', // TODO range might not accurate
            _ => unreachable!()
        }
    }
}