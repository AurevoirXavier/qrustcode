use std::collections::HashMap;

pub struct Encoder {
    micro_mode: bool,

    // modes -> (Version, indicator)
    //
    // Mode:
    //      Numeric      -> 0
    //      Alphanumeric -> 1
    //      Byte         -> 2
    //      Kanji        -> 3
    //      Terminator   -> 4
    //      252 ~ 255 not implemented yet
    modes: HashMap<u8, String>,

    // indicators -> (Version, (Mode, Indicator_size))
    //
    // Version:
    //      M1 ~ M4 -> 1 ~ 4
    //      1 ~ 9   -> 5
    //      10 ~ 26 -> 6
    //      27 ~ 40 -> 7
    //
    // Mode: same as above
    indicators_size: HashMap<u8, [u8; 4]>,
}

impl Encoder {
    pub fn new() -> Encoder {
        let mut encoder = Encoder {
            micro_mode: Default::default(),
            modes: Default::default(),
            indicators_size: Default::default(),
        };
        encoder.set_micro_mode(false);

        encoder
    }

    pub fn set_micro_mode(&mut self, micro_mode: bool) {
        self.micro_mode = micro_mode;

        if micro_mode {
            unreachable!() // TODO
        } else {
            self.modes = [
                (0u8, "0001"), (1, "0010"), (2, "0100"),
                (3, "1000"), (4, "0000"), (255, "0111"),
                (254, "0011"), (253, "1101"), (252, "0101 1001")
            ].iter()
                .map(|&(mode, indicator)| (mode, indicator.to_string()))
                .collect();

            self.indicators_size = [
                (5, [10, 9, 8, 8]),
                (6, [12, 1, 16, 10]),
                (7, [14, 13, 16, 12])
            ].iter()
                .map(|&x| x)
                .collect();
        }
    }

    pub fn encode(&self, mode: &str, version: &str, correction_level: &str, text: &str) {
        let indicator_size = {
            let mode = match mode {
                "Numeric" => 1,
                "Alphanumeric" => 2,
                "Byte" => 3,
                "Kanji" => 4,
                "Terminator" => 0,
                "ECI" => 255,
                "Structured" => 254,
                "Chinese" => 253,
                "FNC1" => 252,
                _ => panic!()
            };
            let indicators_size = self.indicators_size.get(&{
                if let Ok(version) = version.parse::<u8>() {
                    match version {
                        1...9 => 5,
                        10...26 => 6,
                        27...40 => 7,
                        _ => panic!()
                    }
                } else {
                    match version {
                        "M1" => 1,
                        "M2" => 2,
                        "M3" => 3,
                        "M4" => 4,
                        _ => panic!()
                    }
                }
            }).unwrap();

            indicators_size[mode - 1]
        };
    }
}