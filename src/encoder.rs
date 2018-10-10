use std::collections::HashMap;

pub struct Encoder {
    // Number of bits in character count indicator for QR Code
    // indicators_bit's index -> version
    // indicators             -> [indicators' bits in different mode]
    // indicators[mode]       -> indicator's bits
    //
    // version:
    //      micro_mode:
    //          M1 ~ M4 -> 0 ~ 3
    //      normal:
    //          1  ~ 9  -> 0
    //          10 ~ 26 -> 1
    //          27 ~ 40 -> 2
    //
    // mode: same as above
    indicators: [[usize; 4]; 3],

    // Encoding table for Alphanumeric mode
    alphanumeric_table: HashMap<char, usize>,

    // Error correction characteristics for QR Code
    // total_and_correction's index                   -> version
    // total_and_correction[version]                  -> (total number of codewords, [ec_levels])
    // number of error correction codewords[ec_level] -> number of error correction codewords
    //
    // version:
    //      N/A    -> 0
    //      1 ~ 40 -> 1 ~ 40
    //
    // ec_levels:
    //      L -> 0
    //      M -> 1
    //      Q -> 2
    //      H -> 3
    codewords: [[usize; 4]; 40],
}

impl Encoder {
    pub fn new() -> Encoder {
        Encoder {
            indicators: [
                [10, 9, 8, 8],
                [12, 11, 16, 10],
                [14, 13, 16, 12]
            ],
            alphanumeric_table: [
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
                .collect(),
            codewords: [
                [19, 16, 13, 9], [34, 28, 22, 16], [55, 44, 34, 26], [80, 64, 48, 36],
                [108, 86, 62, 46], [136, 108, 76, 60], [156, 124, 88, 66], [194, 154, 110, 86],
                [232, 182, 132, 100], [274, 216, 154, 122], [324, 254, 180, 140], [370, 290, 206, 158],
                [428, 334, 244, 180], [461, 365, 261, 197], [523, 415, 295, 223], [589, 453, 325, 253],
                [647, 507, 367, 283], [721, 563, 397, 313], [795, 627, 445, 341], [861, 669, 485, 385],
                [932, 714, 512, 406], [1006, 782, 568, 442], [1094, 860, 614, 464], [1174, 914, 664, 514],
                [1276, 1000, 718, 538], [1370, 1062, 754, 596], [1468, 1128, 808, 628], [1531, 1193, 871, 661],
                [1631, 1267, 911, 701], [1735, 1373, 985, 745], [1843, 1455, 1033, 793], [1955, 1541, 1115, 845],
                [2071, 1631, 1171, 901], [2191, 1725, 1231, 961], [2306, 1812, 1286, 986], [2434, 1914, 1354, 1054],
                [2566, 1992, 1426, 1096], [2702, 2102, 1502, 1142], [2812, 2216, 1582, 1222], [2956, 2334, 1666, 1276]
            ],
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
        let mut encode = vec![vec![false, false, false, true]];

        encode.push(Encoder::binary(bits_count, len));

        for i in (0..edge).step_by(3) {
            encode.push(Encoder::binary(bits_count, text[i..i + 3].parse().unwrap()));
        }

        match len - edge {
            bits @ 1...2 => encode.push(Encoder::binary(1 + 3 * bits, text[edge..len].parse().unwrap())),
            _ => ()
        }

        encode.concat()
    }

    fn alphanumeric_encode(&self, bits_count: usize, text: &str) -> Vec<bool> {
        let len = text.len();
        let text: Vec<_> = text.chars().collect();
        let mut encode = vec![vec![false, false, true, false]];

        encode.push(Encoder::binary(bits_count, len));

        for i in (0..len >> 1 << 1).step_by(2) {
            encode.push(Encoder::binary(
                11,
                45 * (*self.alphanumeric_table.get(&text[i]).unwrap()) + *self.alphanumeric_table.get(&text[i + 1]).unwrap(),
            ));
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

    pub fn encode(&self, mode: &str, version: &str, ec_level: &str, text: &str) {
        let version = version.parse::<usize>().unwrap() - 1; // index from zero
        let ec_level = match ec_level {
            "L" => 0,
            "M" => 1,
            "Q" => 2,
            "H" => 3,
            _ => panic!()
        };
        let bits_counts = self.indicators[match version {
            0...8 => 0,
            9...25 => 1,
            26...39 => 2,
            _ => panic!()
        }];

        let mut encode = match mode {
            "Numeric" => self.numeric_encode(bits_counts[0], text),
            "Alphanumeric" => self.alphanumeric_encode(bits_counts[1], text),
            "Byte" => self.byte_encode(bits_counts[2], text),
            "Kanji" => self.kanji_encode(bits_counts[3], text),
            "Chinese" => self.chinese_encode(bits_counts[3], text),
            _ => unreachable!() // TODO
        };

        {
            for _ in 0..12 - (4 + encode.len()) % 8 { encode.push(false); } // terminator

            let padding = (self.codewords[version][ec_level] * 8 - encode.len()) / 8;
            let mut padding_bytes = [
                [true, true, true, false, true, true, false, false],
                [false, false, false, true, false, false, false, true]
            ].iter().cycle();

            for _ in 0..padding { encode.extend_from_slice(padding_bytes.next().unwrap()); }
        }

        println!("{:?}", encode);
    }
}