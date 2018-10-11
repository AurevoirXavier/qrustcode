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
const INDICATORS: [[u8; 4]; 3] = [
    [10, 9, 8, 8],
    [12, 11, 16, 10],
    [14, 13, 16, 12]
];

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
//
// codewords[version][ec_level] -> (total number of data codewords for this Version and ec level, ec codewords per block)
const CODEWORDS: [[(u32, u8); 4]; 40] = [
    [(19, 7), (16, 10), (13, 13), (9, 17)], [(34, 10), (28, 16), (22, 22), (16, 28)],
    [(55, 15), (44, 26), (34, 18), (26, 22)], [(80, 20), (64, 18), (48, 26), (36, 16)],
    [(108, 26), (86, 24), (62, 18), (46, 22)], [(136, 18), (108, 16), (76, 24), (60, 28)],
    [(156, 20), (124, 18), (88, 18), (66, 26)], [(194, 24), (154, 22), (110, 22), (86, 26)],
    [(232, 30), (182, 22), (132, 20), (100, 24)], [(274, 18), (216, 26), (154, 24), (122, 28)],
    [(324, 20), (254, 30), (180, 28), (140, 24)], [(370, 24), (290, 22), (206, 26), (158, 28)],
    [(428, 26), (334, 22), (244, 24), (180, 22)], [(461, 30), (365, 24), (261, 20), (197, 24)],
    [(523, 22), (415, 24), (295, 30), (223, 24)], [(589, 24), (453, 28), (325, 24), (253, 30)],
    [(647, 28), (507, 28), (367, 28), (283, 28)], [(721, 30), (563, 26), (397, 28), (313, 28)],
    [(795, 28), (627, 26), (445, 26), (341, 26)], [(861, 28), (669, 26), (485, 30), (385, 28)],
    [(932, 28), (714, 26), (512, 28), (406, 30)], [(1006, 28), (782, 28), (568, 30), (442, 24)],
    [(1094, 30), (860, 28), (614, 30), (464, 30)], [(1174, 30), (914, 28), (664, 30), (514, 30)],
    [(1276, 26), (1000, 28), (718, 30), (538, 30)], [(1370, 28), (1062, 28), (754, 28), (596, 30)],
    [(1468, 30), (1128, 28), (808, 30), (628, 30)], [(1531, 30), (1193, 28), (871, 30), (661, 30)],
    [(1631, 30), (1267, 28), (911, 30), (701, 30)], [(1735, 30), (1373, 28), (985, 30), (745, 30)],
    [(1843, 30), (1455, 28), (1033, 30), (793, 30)], [(1955, 30), (1541, 28), (1115, 30), (845, 30)],
    [(2071, 30), (1631, 28), (1171, 30), (901, 30)], [(2191, 30), (1725, 28), (1231, 30), (961, 30)],
    [(2306, 30), (1812, 28), (1286, 30), (986, 30)], [(2434, 30), (1914, 28), (1354, 30), (1054, 30)],
    [(2566, 30), (1992, 28), (1426, 30), (1096, 30)], [(2702, 30), (2102, 28), (1502, 30), (1142, 30)],
    [(2812, 30), (2216, 28), (1582, 30), (1222, 30)], [(2956, 30), (2334, 28), (1666, 30), (1276, 30)]
];

fn binary(mut bits_count: usize, mut num: u16) -> Vec<u8> {
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

fn alphanumeric_table(b: u8) -> u16 {
    match b {
        48...57 => b as u16 - 48, // '0' ~ '9'
        65...90 => b as u16 - 65, // 'A' ~ 'Z'
        32 => 36,                 // ' '
        36 => 37,                 // '$'
        37 => 38,                 // '%'
        42 => 39,                 // '*'
        43 => 40,                 // '+'
        45 => 41,                 // '-'
        46 => 42,                 // '.'
        47 => 43,                 // '/'
        58 => 44,                 // ':'
        _ => panic!()
    }
}

pub struct Encoder {
    // log and anti log tables
    gf_exp: [u8; 512],
    gf_log: [u8; 256],
}

impl Encoder {
    fn init_gf_tables() -> ([u8; 512], [u8; 256]) {
        let mut gf_exp = [0; 512];
        let mut gf_log = [0; 256];
        let mut x = 1;

        for i in 0..255 {
            gf_exp[i] = x as u8;
            gf_log[x] = i as u8;

            x <<= 1;

            if x & 0x100 != 0 { x ^= 256; }
        }

        for i in 255..512 { gf_exp[i] = gf_exp[i - 255]; }

        (gf_exp, gf_log)
    }

    pub fn new() -> Encoder {
        let (gf_exp, gf_log) = Encoder::init_gf_tables();

        Encoder {
            gf_exp,
            gf_log,
        }
    }

    fn error_correct(&self, mut encode: Vec<u8>, version: usize, ec_level: usize) -> Vec<u8> {
        for _ in 0..12 - (4 + encode.len()) % 8 { encode.push(0); } // terminator

        let re_cws = CODEWORDS[version][ec_level].0 - encode.len() as u32 / 8;

        let mut decimals = vec![];
        for binary in encode.chunks(8) { decimals.push(decimal(binary)); }

        let mut paddings = [236u8, 17].iter().cycle();
        for _ in 0..re_cws { decimals.push(*paddings.next().unwrap()); }

        decimals
    }

    fn numeric_encode(&self, bits_count: u8, text: &str) -> Vec<u8> {
        let len = text.len();
        let edge = len / 3 * 3;
        let mut encode = vec![vec![0, 0, 0, 1]];

        encode.push(binary(bits_count as usize, len as u16));

        for i in (0..edge).step_by(3) {
            encode.push(binary(bits_count as usize, text[i..i + 3].parse().unwrap()));
        }

        match len - edge {
            bits @ 1...2 => encode.push(binary(1 + 3 * bits, text[edge..len].parse().unwrap())),
            _ => ()
        }

        encode.concat()
    }

    fn alphanumeric_encode(&self, bits_count: u8, text: &str) -> Vec<u8> {
        let len = text.len();
        let text = text.as_bytes();
        let mut encode = vec![vec![0, 0, 1, 0]];

        encode.push(binary(bits_count as usize, len as u16));

        for i in (0..len >> 1 << 1).step_by(2) {
            encode.push(binary(
                11,
                45 * alphanumeric_table(text[i]) + alphanumeric_table(text[i + 1]),
            ));
        }

        if len & 1 == 1 { encode.push(binary(6, *text.last().unwrap() as u16)); } else {  }

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
        let bits_counts = INDICATORS[match version {
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
        let ec_cw_per_block = CODEWORDS[version][ec_level].1;

        println!("{:?}", encode);
    }
}