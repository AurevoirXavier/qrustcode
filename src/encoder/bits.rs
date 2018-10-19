pub fn binary(mut bits_count: usize, mut num: u16) -> Vec<u8> {
    let mut binary = vec![];
    binary.resize(bits_count, 0);

    while num != 0 {
        bits_count -= 1;
        binary[bits_count] = (num & 1) as u8;
        num >>= 1;
    }

    binary
}

pub fn decimal(binary: &[u8]) -> u8 {
    let mut decimal = 0;
    for (exp, bit) in binary.iter().rev().enumerate() { decimal += bit << exp; }

    decimal
}

use super::Encoder;
impl Encoder {
    pub fn decimal_data(&mut self) -> &mut Encoder {
        use super::qrcode_info::CAPACITIES;

        let data = &mut self.data;

        // terminator = 4
        // 12 = 8 + terminator
        // re = (4 + data.len()) % 8
        for _ in 0..12 - (4 + data.len()) % 8 { data.push(0); }

        let re_cws = (CAPACITIES[self.version][self.ec_level] - data.len() as u16) / 8;

        let mut decimals = vec![];
        for binary in data.chunks(8) { decimals.push(decimal(binary)); }

        let mut paddings = [236u8, 17].iter().cycle();
        for _ in 0..re_cws { decimals.push(*paddings.next().unwrap()); }

        *data = decimals;

        self
    }
}