mod bits;
mod matrix;
mod encode;
mod error_correct;
mod qrcode_info;

pub struct Encoder {
    data: Vec<u8>,
    ec_data: Vec<u8>,
    mode: u8,
    version: usize,
    ec_level: usize,
}

impl Encoder {
    pub fn new() -> Encoder {
        Encoder {
            data: vec![],
            ec_data: vec![],
            mode: 0,
            version: 0,
            ec_level: 0,
        }
    }

    pub fn mode(mut self, mode: &str) -> Encoder {
        self.mode = match mode {
            "Numeric" => 0,
            "Alphanumeric" => 1,
            "Byte" => 2,
            "Kanji" => 3,
            "Chinese" => 4,
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