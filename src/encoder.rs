use std::collections::HashMap;

pub struct Encoder {
    // modes -> (Version, indicator)
    modes: HashMap<String, String>,

    // indicators -> (Mode, (Version, Indicator_size))
    //
    // Version:
    //      M1 ~ M4 -> 1 ~ 4
    //      1 ~ 9   -> 5
    //      10 ~ 26 -> 6
    //      27 ~ 40 -> 7
    //
    // Mode:
    //      Numeric      -> 1
    //      Alphanumeric -> 2
    //      Byte         -> 3
    //      Kanji        -> 4
    indicators_size: HashMap<String, [(i8, u8); 4]>,
}

impl Encoder {
    pub fn new() -> Encoder {
        Encoder {
            modes: [
                ("ECI", "0111"), ("Numeric", "0001"), ("Alphanumeric", "0010"),
                ("Byte", "0100"), ("Kanji", "1000"), ("Structured", "0011"),
                ("Chinese", "1101"), ("FNC1", "0101 1001"), ("Terminator", "0000")
            ].iter()
                .map(|&(mode, indicator)| (mode.to_string(), indicator.to_string()))
                .collect(),
            indicators_size: [
                (1, (1, 3))
            ].iter()
                .collect(),
        }
    }

    pub fn encode(&self, mode: &str, version: &str, correction_level: &str) {}
}