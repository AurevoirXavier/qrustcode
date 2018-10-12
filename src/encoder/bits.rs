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

use super::{qrcode_info::CAPACITIES, Encoder};
impl Encoder {
    pub fn decimal_data(&mut self) -> &mut Encoder {
        let data = &mut self.data;

        for _ in 0..12 - (4 + data.len()) % 8 { data.push(0); } // terminator

        let re_cws = CAPACITIES[self.version][self.ec_level] - data.len() as u32 / 8;

        let mut decimals = vec![];
        for binary in data.chunks(8) { decimals.push(decimal(binary)); }

        let mut paddings = [236u8, 17].iter().cycle();
        for _ in 0..re_cws { decimals.push(*paddings.next().unwrap()); }

        *data = decimals;

        self
    }
}