// Number of bits in character count indicator for QR Code
// indicators_bit's index -> version
// indicators             -> [indicators' bits in different mode]
// indicators[mode]       -> indicator's bits
//
// version:
//      micro_mode:
//          M1 ~ M4 -> TODO
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
//      1 ~ 40 -> 0 ~ 39
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

// log and anti log tables in Galois Field - GF(2^8)
// store double GF_EXP table to guarantee GF_EXP[(GF_LOG[x] + GF_LOG[y])] not out of index
const GF_EXP: [u8; 512] = [
    1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38, 76, 152, 45, 90, 180, 117, 234,
    201, 143, 3, 6, 12, 24, 48, 96, 192, 157, 39, 78, 156, 37, 74, 148, 53, 106, 212, 181, 119, 238,
    193, 159, 35, 70, 140, 5, 10, 20, 40, 80, 160, 93, 186, 105, 210, 185, 111, 222, 161, 95, 190,
    97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30, 60, 120, 240, 253, 231, 211, 187, 107, 214,
    177, 127, 254, 225, 223, 163, 91, 182, 113, 226, 217, 175, 67, 134, 17, 34, 68, 136, 13, 26, 52,
    104, 208, 189, 103, 206, 129, 31, 62, 124, 248, 237, 199, 147, 59, 118, 236, 197, 151, 51, 102,
    204, 133, 23, 46, 92, 184, 109, 218, 169, 79, 158, 33, 66, 132, 21, 42, 84, 168, 77, 154, 41,
    82, 164, 85, 170, 73, 146, 57, 114, 228, 213, 183, 115, 230, 209, 191, 99, 198, 145, 63, 126,
    252, 229, 215, 179, 123, 246, 241, 255, 227, 219, 171, 75, 150, 49, 98, 196, 149, 55, 110, 220,
    165, 87, 174, 65, 130, 25, 50, 100, 200, 141, 7, 14, 28, 56, 112, 224, 221, 167, 83, 166, 81,
    162, 89, 178, 121, 242, 249, 239, 195, 155, 43, 86, 172, 69, 138, 9, 18, 36, 72, 144, 61, 122,
    244, 245, 247, 243, 251, 235, 203, 139, 11, 22, 44, 88, 176, 125, 250, 233, 207, 131, 27, 54,
    108, 216, 173, 71, 142, 1, 2, 4, 8, 16, 32, 64, 128, 29, 58, 116, 232, 205, 135, 19, 38, 76,
    152, 45, 90, 180, 117, 234, 201, 143, 3, 6, 12, 24, 48, 96, 192, 157, 39, 78, 156, 37, 74, 148,
    53, 106, 212, 181, 119, 238, 193, 159, 35, 70, 140, 5, 10, 20, 40, 80, 160, 93, 186, 105, 210,
    185, 111, 222, 161, 95, 190, 97, 194, 153, 47, 94, 188, 101, 202, 137, 15, 30, 60, 120, 240,
    253, 231, 211, 187, 107, 214, 177, 127, 254, 225, 223, 163, 91, 182, 113, 226, 217, 175, 67,
    134, 17, 34, 68, 136, 13, 26, 52, 104, 208, 189, 103, 206, 129, 31, 62, 124, 248, 237, 199, 147,
    59, 118, 236, 197, 151, 51, 102, 204, 133, 23, 46, 92, 184, 109, 218, 169, 79, 158, 33, 66, 132,
    21, 42, 84, 168, 77, 154, 41, 82, 164, 85, 170, 73, 146, 57, 114, 228, 213, 183, 115, 230, 209,
    191, 99, 198, 145, 63, 126, 252, 229, 215, 179, 123, 246, 241, 255, 227, 219, 171, 75, 150, 49,
    98, 196, 149, 55, 110, 220, 165, 87, 174, 65, 130, 25, 50, 100, 200, 141, 7, 14, 28, 56, 112,
    224, 221, 167, 83, 166, 81, 162, 89, 178, 121, 242, 249, 239, 195, 155, 43, 86, 172, 69, 138,
    9, 18, 36, 72, 144, 61, 122, 244, 245, 247, 243, 251, 235, 203, 139, 11, 22, 44, 88, 176, 125,
    250, 233, 207, 131, 27, 54, 108, 216, 173, 71, 142, 1, 2
];
const GF_LOG: [u8; 256] = [
    0, 0, 1, 25, 2, 50, 26, 198, 3, 223, 51, 238, 27, 104, 199, 75, 4, 100, 224, 14, 52, 141, 239,
    129, 28, 193, 105, 248, 200, 8, 76, 113, 5, 138, 101, 47, 225, 36, 15, 33, 53, 147, 142, 218,
    240, 18, 130, 69, 29, 181, 194, 125, 106, 39, 249, 185, 201, 154, 9, 120, 77, 228, 114, 166, 6,
    191, 139, 98, 102, 221, 48, 253, 226, 152, 37, 179, 16, 145, 34, 136, 54, 208, 148, 206, 143,
    150, 219, 189, 241, 210, 19, 92, 131, 56, 70, 64, 30, 66, 182, 163, 195, 72, 126, 110, 107, 58,
    40, 84, 250, 133, 186, 61, 202, 94, 155, 159, 10, 21, 121, 43, 78, 212, 229, 172, 115, 243, 167,
    87, 7, 112, 192, 247, 140, 128, 99, 13, 103, 74, 222, 237, 49, 197, 254, 24, 227, 165, 153, 119,
    38, 184, 180, 124, 17, 68, 146, 217, 35, 32, 137, 46, 55, 63, 209, 91, 149, 188, 207, 205, 144,
    135, 151, 178, 220, 252, 190, 97, 242, 86, 211, 171, 20, 42, 93, 158, 132, 60, 57, 83, 71, 109,
    65, 162, 31, 45, 67, 216, 183, 123, 164, 118, 196, 23, 73, 236, 127, 12, 111, 246, 108, 161, 59,
    82, 41, 157, 85, 170, 251, 96, 134, 177, 187, 204, 62, 90, 203, 89, 95, 176, 156, 169, 160, 81,
    11, 245, 22, 235, 122, 117, 44, 215, 79, 174, 213, 233, 230, 231, 173, 232, 116, 214, 244, 234,
    168, 80, 88, 175
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
        b'0'...b'9' => (b - 48) as u16, // 48 = b'0'
        b'A'...b'Z' => (b - 55) as u16, // 55 = b'A' - 10
        b' ' => 36,
        b'$' => 37,
        b'%' => 38,
        b'*' => 39,
        b'+' => 40,
        b'-' => 41,
        b'.' => 42,
        b'/' => 43,
        b':' => 44,
        _ => panic!()
    }
}

pub struct Encoder {
    message: &'static str,
    data: Vec<u8>,
    mode: u8,
    version: usize,
    ec_level: usize,
}

impl Encoder {
    pub fn new(mode: &str, version: usize, ec_level: &str, message: &'static str) -> Encoder {
        Encoder {
            message,
            data: vec![],
            mode: match mode {
                "Numeric" => 0,
                "Alphanumeric" => 1,
                "Byte" => 2,
                "Kanji" => 3,
                "Chinese" => 4,
                _ => panic!()
            },
            version: version - 1, // index from 0
            ec_level: match ec_level {
                "L" => 0,
                "M" => 1,
                "Q" => 2,
                "H" => 3,
                _ => panic!()
            },
        }
    }

    fn fill_blank(&mut self) -> &mut Encoder {
        let data = &mut self.data;

        for _ in 0..12 - (4 + data.len()) % 8 { data.push(0); } // terminator

        let re_cws = CODEWORDS[self.version][self.ec_level].0 - data.len() as u32 / 8;

        let mut decimals = vec![];
        for binary in data.chunks(8) { decimals.push(decimal(binary)); }

        let mut paddings = [236u8, 17].iter().cycle();
        for _ in 0..re_cws { decimals.push(*paddings.next().unwrap()); }

        *data = decimals;

        self
    }

    fn error_correction(&mut self) {
//        let mut message = vec![];
//        message.resize(self.message.len() + )
    }

    fn numeric_encode(&mut self, bits_count: usize) -> &mut Encoder {
        let message = self.message;
        let len = message.len();
        let edge = len / 3 * 3;
        let mut data = vec![vec![0, 0, 0, 1]];

        data.push(binary(bits_count, len as u16));

        for i in (0..edge).step_by(3) {
            data.push(binary(bits_count, message[i..i + 3].parse().unwrap()));
        }

        match len - edge {
            bits @ 1...2 => data.push(binary(1 + 3 * bits, message[edge..len].parse().unwrap())),
            0 => (),
            _ => panic!()
        }

        self.data = data.concat();

        self
    }

    fn alphanumeric_encode(&mut self, bits_count: usize) -> &mut Encoder {
        let message = self.message.as_bytes();
        let len = message.len();
        let mut data = vec![vec![0, 0, 1, 0]];

        data.push(binary(bits_count, len as u16));

        for i in (0..len >> 1 << 1).step_by(2) {
            data.push(binary(
                11,
                45 * alphanumeric_table(message[i]) + alphanumeric_table(message[i + 1]),
            ));
        }

        if len & 1 == 1 { data.push(binary(6, alphanumeric_table(*message.last().unwrap()))); }

        self.data = data.concat();

        self
    }

    fn byte_encode(&mut self, bits_count: usize) -> &mut Encoder {
        self
    }

    fn kanji_encode(&mut self, bits_count: usize) -> &mut Encoder {
        self
    }

    fn chinese_encode(&mut self, bits_count: usize) -> &mut Encoder {
        self
    }

    pub fn encode(&mut self) {
        let bits_counts = INDICATORS[match self.version {
            0...8 => 0,
            9...25 => 1,
            26...39 => 2,
            _ => panic!()
        }];

        match self.mode {
            0 => self.numeric_encode(bits_counts[0] as usize),
            1 => self.alphanumeric_encode(bits_counts[1] as usize),
            2 => self.byte_encode(bits_counts[2] as usize),
            3 => self.kanji_encode(bits_counts[3] as usize),
            4 => self.chinese_encode(bits_counts[3] as usize),
            _ => panic!()
        }
            .fill_blank()
            .error_correction();

        let ec_cw_per_block = CODEWORDS[self.version][self.ec_level].1;

        println!("{:?}", self.data);
    }
}