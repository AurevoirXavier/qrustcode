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


// Total Number of Data Codewords for this Version and EC Level
// codewords[version][ec_level] -> total number of data codewords for this Version and ec level
//
// version:
//      1 ~ 40 -> 0 ~ 39
//
// ec_levels:
//      L -> 0
//      M -> 1
//      Q -> 2
//      H -> 3
pub const CAPACITIES: [[u32; 4]; 40] = [
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
];


// EC Codewords Per Block
//
// version, ec_level: same as above
pub const EC_CW_PER_BLOCKS: [[u8; 4]; 40] = [
    [7, 10, 13, 17], [10, 16, 22, 28], [15, 26, 18, 22], [20, 18, 26, 16], [26, 24, 18, 22],
    [18, 16, 24, 28], [20, 18, 18, 26], [24, 22, 22, 26], [30, 22, 20, 24], [18, 26, 24, 28],
    [20, 30, 28, 24], [24, 22, 26, 28], [26, 22, 24, 22], [30, 24, 20, 24], [22, 24, 30, 24],
    [24, 28, 24, 30], [28, 28, 28, 28], [30, 26, 28, 28], [28, 26, 26, 26], [28, 26, 30, 28],
    [28, 26, 28, 30], [28, 28, 30, 24], [30, 28, 30, 30], [30, 28, 30, 30], [26, 28, 30, 30],
    [28, 28, 28, 30], [30, 28, 30, 30], [30, 28, 30, 30], [30, 28, 30, 30], [30, 28, 30, 30],
    [30, 28, 30, 30], [30, 28, 30, 30], [30, 28, 30, 30], [30, 28, 30, 30], [30, 28, 30, 30],
    [30, 28, 30, 30], [30, 28, 30, 30], [30, 28, 30, 30], [30, 28, 30, 30], [30, 28, 30, 30]
];