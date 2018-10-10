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
    indicators: [[u8; 4]; 3],

    // Encoding table for Alphanumeric mode
    alphanumeric_table: HashMap<char, u8>,

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
    codewords: [[u32; 4]; 40],

    // log and anti log tables
    gf_exp: [u8; 512],
    gf_log: [u8; 256],
}

impl Encoder {
    fn init_gf_tables() -> ([u8; 512], [u8; 256]) {
        let mut gf_exp = [0; 512];
        let mut gf_log = [0; 256];
        let mut x = 1u8;

        for i in 0..255 {
            gf_exp[i] = x;
            gf_log[x] = i;

            x <<= 1;

            if x & 0x100 != 0 { x ^= 256; }
        }

        for i in 255..512 { gf_exp[i] = gf_exp[i - 255]; }

        (gf_exp, gf_log)
    }

    pub fn new() -> Encoder {
        let (gf_exp, gf_log) = Encoder::init_gf_tables();

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
            gf_exp,
            gf_log,
        }
    }

    fn binary(mut bits_count: usize, mut num: u32) -> Vec<u8> {
        let mut binary = vec![];
        binary.resize(bits_count, 0);

        while num != 0 {
            bits_count -= 1;
            binary[bits_count] = (num & 1) as u8;
            num >>= 1;
        }

        binary
    }

    fn decimal(binary: &[u8]) -> u8 {
        let mut decimal = 0;
        for (exp, bit) in binary.iter().rev().enumerate() { decimal += bit << exp; }

        decimal
    }

    fn error_correct(&self, mut encode: Vec<u8>, version: usize, ec_level: usize) -> Vec<u8> {
        for _ in 0..12 - (4 + encode.len()) % 8 { encode.push(0); } // terminator

        let re_codewords = self.codewords[version][ec_level] - encode.len() as u32 / 8;

        let mut decimals = vec![];
        for binary in encode.chunks(8) { decimals.push(Encoder::decimal(binary)); }

        let mut paddings = [236u8, 17].iter().cycle();
        for _ in 0..re_codewords { decimals.push(*paddings.next().unwrap()); }

        decimals
    }

    fn numeric_encode(&self, bits_count: u8, text: &str) -> Vec<u8> {
        let len = text.len();
        let edge = len / 3 * 3;
        let mut encode = vec![vec![0, 0, 0, 1]];

        encode.push(Encoder::binary(bits_count as usize, len as u32));

        for i in (0..edge).step_by(3) {
            encode.push(Encoder::binary(bits_count as usize, text[i..i + 3].parse().unwrap()));
        }

        match len - edge {
            bits @ 1...2 => encode.push(Encoder::binary(1 + 3 * bits, text[edge..len].parse().unwrap())),
            _ => ()
        }

        encode.concat()
    }

    fn alphanumeric_encode(&self, bits_count: u8, text: &str) -> Vec<u8> {
        let len = text.len();
        let text: Vec<_> = text.chars().collect();
        let mut encode = vec![vec![0, 0, 1, 0]];

        encode.push(Encoder::binary(bits_count as usize, len as u32));

        for i in (0..len >> 1 << 1).step_by(2) {
            encode.push(Encoder::binary(
                11,
                45 * (*self.alphanumeric_table.get(&text[i]).unwrap()) as u32 + *self.alphanumeric_table.get(&text[i + 1]).unwrap() as u32,
            ));
        }

        if len & 1 == 1 { encode.push(Encoder::binary(6, *self.alphanumeric_table.get(&text[len - 1]).unwrap() as u32)); }

        encode.concat()
    }

    fn byte_encode(&self, bits_count: u8, text: &str) -> Vec<u8> {
        vec![]
    }

    fn kanji_encode(&self, bits_count: u8, text: &str) -> Vec<u8> {
        vec![]
    }

    fn chinese_encode(&self, bits_count: u8, text: &str) -> Vec<u8> {
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

        let encode = self.error_correct(
            match mode {
                "Numeric" => self.numeric_encode(bits_counts[0], text),
                "Alphanumeric" => self.alphanumeric_encode(bits_counts[1], text),
                "Byte" => self.byte_encode(bits_counts[2], text),
                "Kanji" => self.kanji_encode(bits_counts[3], text),
                "Chinese" => self.chinese_encode(bits_counts[3], text),
                _ => unreachable!() // TODO
            },
            version,
            ec_level,
        );

        println!("{:?}", encode);
    }
}