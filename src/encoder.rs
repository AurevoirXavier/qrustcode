pub struct Encoder {
    micro_mode: bool,

    // modes' index -> mode
    // modes        -> indicator
    //
    // mode:
    //      Numeric      -> 0
    //      Alphanumeric -> 1
    //      Byte         -> 2
    //      Kanji        -> 3
    //      252 ~ 255 not implemented yet
    modes: [String; 4],

    // indicators_size's index -> version
    // indicators              -> [indicators' size in different mode]
    // indicators[mode]        -> indicator's size
    //
    // version:
    //      micro_mode:
    //          M1 ~ M4 -> 0 ~ 3
    //      normal:
    //          1 ~ 9   -> 0
    //          10 ~ 26 -> 1
    //          27 ~ 40 -> 2
    //
    // mode: same as above
    indicators_size: [[u8; 4]; 3],
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
                String::from("0001"),
                String::from("0010"),
                String::from("0100"),
                String::from("1000")
            ];
            self.indicators_size = [
                [10, 9, 8, 8],
                [12, 1, 16, 10],
                [14, 13, 16, 12]
            ]
        }
    }

    pub fn encode(&self, mode: &str, version: &str, correction_level: &str, text: &str) {
        let indicator_size = {
            let mode = match mode {
                "Numeric" => 0,
                "Alphanumeric" => 1,
                "Byte" => 2,
                "Kanji" => 3,
                _ => unreachable!() // TODO
            };
            let indicators_size = self.indicators_size[{
                if self.micro_mode {
                    unreachable!() // TODO
                } else {
                    match version.parse::<u8>().unwrap() {
                        1...9 => 0,
                        10...26 => 1,
                        27...40 => 2,
                        _ => panic!()
                    }
                }
            }];

            indicators_size[mode]
        };
    }
}