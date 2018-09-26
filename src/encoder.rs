enum Version {
    M2,
    M3,
    M4,
    Version(u8)
}

enum EncodingMode {
    Numeric,
    Alphanumeric,
    EightBits,
    Kanji,
}

enum CorrectionLevel { L, M, Q, H }

struct Encoder {
    version: Version,
}

impl Encoder {
    fn new(version: u8, encoding_mode: EncodingMode) -> Encoder {
        match version {
            1...40 => Encoder {
                version,
            },
            _ => panic!("Invalid version.")
        }
    }

    fn encode(&self, encoding_mode: EncodingMode, correction_level: CorrectionLevel, data: &str) {
        {
            use self::CorrectionLevel::*;
            match correction_level {
                L => (),
                M => (),
                Q => (),
                H => (),
            }
        }

        {
            use self::EncodingMode::*;
            match encoding_mode {
                Alphanumeric => (),
                EightBits => (),
                Kanji => (),
                Numeric => (),
            }
        }
    }
}