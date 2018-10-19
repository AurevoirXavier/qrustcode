mod bits;
mod matrix;
mod mode;
mod encode;
mod error_correct;
mod qrcode_info;

use self::mode::Mode;

pub struct Encoder {
    data: Vec<u8>,
    mode: Mode,

    // version:
    //     micro_mode:
    //         M1 ~ M4 -> TODO
    //     normal:
    //         1  ~ 9  -> 0
    //         10 ~ 26 -> 1
    //         27 ~ 40 -> 2
    version: usize,

    // ec_levels:
    //     L -> 0
    //     M -> 1
    //     Q -> 2
    //     H -> 3
    ec_level: usize,
}

impl Encoder {
    pub fn new() -> Encoder {
        Encoder {
            data: vec![],
            mode: Mode::Unknown,
            version: 255,
            ec_level: 0,
        }
    }

    pub fn mode(mut self, mode: &str) -> Encoder {
        self.mode = match mode {
            "Numeric" => Mode::Numeric,
            "Alphanumeric" => Mode::Alphanumeric,
            "ByteISO88591" => Mode::ByteISO88591,
            "ByteUTF8" => Mode::ByteUTF8,
            "Kanji" => Mode::Kanji,
            "Chinese" => Mode::Chinese,
            _ => panic!()
        };

        self
    }

    pub fn version(mut self, version: usize) -> Encoder {
        self.version = version - 1; // index from 0

        self
    }

    pub fn ec_level(mut self, ec_level: &str) -> Encoder {
        self.ec_level = match ec_level {
            "L" => 0,
            "M" => 1,
            "Q" => 2,
            "H" => 3,
            _ => panic!()
        };

        self
    }
}