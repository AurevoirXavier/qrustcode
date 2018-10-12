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
pub const INDICATORS: [[u8; 4]; 3] = [
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
pub const CODEWORDS: [[(u32, u8); 4]; 40] = [
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

pub fn alphanumeric_table(b: u8) -> u16 {
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