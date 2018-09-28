use std::collections::HashMap;

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
    //      Chinese      -> 4
    //      252 ~ 255 not implemented yet
    // encoding_modes: [[bool; 4]; 5],

    // indicators_bit's index -> version
    // indicators              -> [indicators' bits in different mode]
    // indicators[mode]        -> indicator's bits
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
    indicators_bits: [[usize; 4]; 3],

    alphanumeric_table: HashMap<char, usize>,
}

impl Encoder {
    pub fn new() -> Encoder {
        let mut encoder = Encoder {
            micro_mode: Default::default(),
            // encoding_modes: Default::default(),
            indicators_bits: Default::default(),
            alphanumeric_table: Default::default(),
        };
        encoder.set_micro_mode(false);

        encoder
    }

    pub fn set_micro_mode(&mut self, micro_mode: bool) {
        self.micro_mode = micro_mode;

        if micro_mode {
            unreachable!() // TODO
        } else {
            // self.encoding_modes = [
            //     [false, false, false, true],
            //     [false, false, true, false],
            //     [false, true, false, false],
            //     [true, false, false, false],
            //     [true, true, false, true]
            // ];

            self.indicators_bits = [
                [10, 9, 8, 8],
                [12, 1, 16, 10],
                [14, 13, 16, 12]
            ];

            self.alphanumeric_table = [
                ('0', 0), ('1', 1), ('2', 2), ('3', 3), ('4', 4), ('5', 5),
                ('6', 6), ('7', 7), ('8', 8), ('9', 9),
                ('A', 10), ('B', 11), ('C', 12), ('D', 13), ('E', 14), ('F', 15),
                ('G', 16), ('H', 17), ('I', 18), ('J', 19), ('K', 20), ('L', 21),
                ('M', 22), ('N', 23), ('O', 24), ('P', 25), ('Q', 26), ('R', 27),
                ('S', 28), ('T', 29), ('U', 30), ('V', 31), ('W', 32), ('X', 33),
                ('Y', 34), ('Z', 35),
                (' ', 36), ('$', 37), ('%', 38), ('*', 39), ('+', 40), ('-', 41),
                ('.', 42), ('/', 43), (':', 44)
            ].iter()
                .map(|&t| t)
                .collect()
        }
    }

    fn binary(mut bits: usize, mut num: usize) -> Vec<bool> {
        let mut binary = vec![];
        binary.resize(bits, false);

        while num != 0 {
            bits -= 1;

            binary[bits] = match num & 1 {
                0 => false,
                1 => true,
                _ => unreachable!()
            };

            num >>= 1;
        }

        binary
    }

    fn numeric_encode(&self, bits: usize, text: &str) -> Vec<bool> {
        let len = text.len();
        let mut encode = if self.micro_mode {
            unreachable!() // TODO
        } else { vec![vec![false, false, false, true]] };

        encode.push(Encoder::binary(bits, len));

        for i in (0..).step_by(3) {
            match len - i {
                bits @ 1...2 => {
                    encode.push(Encoder::binary(bits * 3 + 1, text[i..len].parse().unwrap()));
                    println!("{:?}", encode.concat());
                    return encode.concat();
                }
                _ => encode.push(Encoder::binary(bits, text[i..i + 3].parse().unwrap()))
            }
        }

        unreachable!()
    }

    fn alphanumeric_encode(&self, indicator_bits: usize, text: &str) -> Vec<bool> {
        vec![]
    }

    fn byte_encode(&self, indicator_bits: usize, text: &str) -> Vec<bool> {
        vec![]
    }

    fn kanji_encode(&self, indicator_bits: usize, text: &str) -> Vec<bool> {
        vec![]
    }

    fn chinese_encode(&self, indicator_bits: usize, text: &str) -> Vec<bool> {
        vec![]
    }

    pub fn encode(&self, mode: &str, version: &str, correction_level: &str, text: &str) {
        let indicators_bits = self.indicators_bits[{
            if self.micro_mode {
                unreachable!() // TODO
            } else {
                match version.parse::<usize>().unwrap() {
                    1...9 => 0,
                    10...26 => 1,
                    27...40 => 2,
                    _ => panic!()
                }
            }
        }];

        let mode = match mode {
            "Numeric" => self.numeric_encode(indicators_bits[0], text),
            "Alphanumeric" => self.alphanumeric_encode(indicators_bits[1], text),
            "Byte" => self.byte_encode(indicators_bits[2], text),
            "Kanji" => self.kanji_encode(indicators_bits[3], text),
            "Chinese" => self.chinese_encode(indicators_bits[3], text),
            _ => unreachable!() // TODO
        };
    }
}