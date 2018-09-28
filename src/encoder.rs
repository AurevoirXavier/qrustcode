use std::collections::HashMap;

enum Indicators {
    Empty,
    // TODO
    MicroMode([[usize; 4]; 3]),
    Normal([[usize; 4]; 3]),
}

enum Codewords {
    Empty,
    // TODO
    MicroMode([(usize, [usize; 4]); 40]),
    Normal([(usize, [usize; 4]); 41]),
}

pub struct Encoder {
    // Switch for if using Micro QR Code
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
    //      252 ~ 255 not implement yet
    // encoding_modes: [[bool; 4]; 5],

    // Number of bits in character count indicator for QR Code
    // indicators_bit's index -> version
    // indicators             -> [indicators' bits in different mode]
    // indicators[mode]       -> indicator's bits
    //
    // version:
    //      micro_mode:
    //          M1 ~ M4 -> 0 ~ 3
    //      normal:
    //          1  ~ 9 -> 0
    //          10 ~ 26 -> 1
    //          27 ~ 40 -> 2
    //
    // mode: same as above
    indicators: Indicators,

    // Encoding table for Alphanumeric mode
    alphanumeric_table: HashMap<char, usize>,

    // Error correction characteristics for QR Code
    // total_and_correction's index                                 -> version
    // total_and_correction[version]                                -> (total number of codewords, [error_correction_levels])
    // number of error correction codewords[error_correction_level] -> number of error correction codewords
    //
    // version:
    //      N/A    -> 0
    //      1 ~ 40 -> 1 ~ 40
    //
    // error_correction_levels:
    //      L -> 0
    //      M -> 1
    //      Q -> 2
    //      H -> 3
    codewords: Codewords,
}

impl Encoder {
    pub fn new() -> Encoder {
        let mut encoder = Encoder {
            micro_mode: false,
            indicators: Indicators::Empty,
            alphanumeric_table: HashMap::new(),
            codewords: Codewords::Empty,
        };
        encoder.set_micro_mode(false);

        encoder
    }

    pub fn set_micro_mode(&mut self, micro_mode: bool) {
        self.micro_mode = micro_mode;

        if micro_mode {
            unreachable!() // TODO
        } else {
            self.indicators = Indicators::Normal([
                [10, 9, 8, 8],
                [12, 11, 16, 10],
                [14, 13, 16, 12]
            ]);

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
                .collect();

            self.codewords = Codewords::Normal([
                (0, [0, 0, 0, 0]),
                (26, [7, 10, 13, 17]), (44, [10, 16, 22, 28]), (70, [15, 26, 36, 44]), (100, [20, 36, 52, 64]),
                (134, [26, 48, 72, 88]), (172, [36, 64, 96, 112]), (196, [40, 72, 108, 130]), (242, [48, 88, 132, 156]),
                (292, [60, 110, 160, 192]), (346, [72, 130, 192, 224]), (404, [80, 150, 224, 264]), (466, [96, 176, 260, 308]),
                (532, [104, 198, 288, 352]), (581, [120, 216, 320, 384]), (655, [132, 240, 360, 432]), (733, [144, 280, 408, 480]),
                (815, [168, 308, 448, 532]), (901, [180, 338, 504, 588]), (991, [196, 364, 546, 650]), (1085, [224, 416, 600, 700]),
                (1156, [224, 442, 644, 750]), (1258, [252, 476, 690, 816]), (1364, [270, 504, 750, 900]), (1474, [300, 560, 810, 960]),
                (1588, [312, 588, 810, 960]), (1706, [336, 644, 952, 1110]), (1828, [360, 700, 1020, 1200]), (1921, [390, 728, 1050, 1260]),
                (2051, [420, 784, 1140, 1350]), (2185, [450, 812, 1200, 1440]), (2323, [480, 868, 1290, 1530]), (2456, [510, 924, 1350, 1620]),
                (2611, [540, 980, 1440, 1710]), (2761, [570, 1036, 1530, 1800]), (2876, [570, 1064, 1590, 1890]), (3034, [600, 1120, 1680, 1980]),
                (3196, [630, 1204, 1770, 2100]), (3362, [660, 1260, 1860, 2220]), (3532, [720, 1316, 1950, 2310]), (3706, [750, 1372, 2040, 2430]),
            ])
        }
    }

    fn binary(mut bits_count: usize, mut num: usize) -> Vec<bool> {
        let mut binary = vec![];
        binary.resize(bits_count, false);

        while num != 0 {
            bits_count -= 1;

            binary[bits_count] = match num & 1 {
                0 => false,
                1 => true,
                _ => unreachable!()
            };

            num >>= 1;
        }

        binary
    }

    fn numeric_encode(&self, bits_count: usize, text: &str) -> Vec<bool> {
        let len = text.len();
        let edge = len / 3 * 3;
        let mut encode = if self.micro_mode {
            unreachable!() // TODO
        } else { vec![vec![false, false, false, true]] };

        encode.push(Encoder::binary(bits_count, len));

        for i in (0..edge).step_by(3) {
            encode.push(Encoder::binary(bits_count, text[i..i + 3].parse().unwrap()));
        }

        match len - edge {
            bits @ 1...2 => encode.push(Encoder::binary(1 + 3 * bits, text[edge..len].parse().unwrap())),
            _ => ()
        }

//        println!("{:?}", encode.concat());
        encode.concat()
    }

    fn alphanumeric_encode(&self, bits_count: usize, text: &str) -> Vec<bool> {
        let len = text.len();
        let text: Vec<_> = text.chars().collect();
        let mut encode = if self.micro_mode {
            unreachable!() // TODO
        } else { vec![vec![false, false, true, false]] };

        encode.push(Encoder::binary(bits_count, len));

        for i in (0..len >> 1 << 1).step_by(2) {
            encode.push(Encoder::binary(
                11,
                45 * (*self.alphanumeric_table.get(&text[i]).unwrap()) + *self.alphanumeric_table.get(&text[i + 1]).unwrap()));
        }

        if len & 1 == 1 { encode.push(Encoder::binary(6, *self.alphanumeric_table.get(&text[len - 1]).unwrap())); }

        encode.concat()
    }

    fn byte_encode(&self, bits_count: usize, text: &str) -> Vec<bool> {
        vec![]
    }

    fn kanji_encode(&self, bits_count: usize, text: &str) -> Vec<bool> {
        vec![]
    }

    fn chinese_encode(&self, bits_count: usize, text: &str) -> Vec<bool> {
        vec![]
    }

    pub fn encode(&self, mode: &str, version: &str, error_correction_level: &str, text: &str) {
        let version: usize = version.parse().unwrap();
        let bits_count = match self.indicators {
            Indicators::Normal(indicators) => indicators[match version {
                1...9 => 0,
                10...26 => 1,
                27...40 => 2,
                _ => panic!()
            }],
            Indicators::MicroMode(indicators) => unreachable!(), // TODO
            _ => unreachable!()
        };

        let mut encode = match mode {
            "Numeric" => self.numeric_encode(bits_count[0], text),
            "Alphanumeric" => self.alphanumeric_encode(bits_count[1], text),
            "Byte" => self.byte_encode(bits_count[2], text),
            "Kanji" => self.kanji_encode(bits_count[3], text),
            "Chinese" => self.chinese_encode(bits_count[3], text),
            _ => unreachable!() // TODO
        };

        let terminator = if self.micro_mode {
            unreachable!() // TODO
        } else { 4 };

        for _ in 0..terminator + encode.len() % 8 { encode.push(false); }

        {
            let padding = (match self.codewords {
                Codewords::Normal(codewords) => codewords[version].0 * 4,
                Codewords::MicroMode(codewords) => unreachable!(), // TODO
                _ => unreachable!()
            } - encode.len()) / 8;
            let mut padding_bytes = [[true, true, true, false, true, true, false, false], [false, false, false, true, false, false, false, true]].iter().cycle();

            for _ in 0..padding { encode.extend_from_slice(padding_bytes.next().unwrap()); }
        }

        println!("{:?}", encode);
    }
}