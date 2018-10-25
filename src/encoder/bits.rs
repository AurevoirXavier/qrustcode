use super::Encoder;

pub fn push_binary(binary: &mut Vec<u8>, bits_count: usize, mut num: u16) {
    let mut len = binary.len() + bits_count;

    binary.resize(len, 0);

    while num != 0 {
        len -= 1;
        binary[len] = (num & 1) as u8;
        num >>= 1;
    }
}

#[test]
fn test_push_binary() {
    let mut binary = vec![1, 0, 0, 1];
    push_binary(&mut binary, 8, 13);

    assert_eq!(binary, vec![1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 1]);
}

pub fn decimal(binary: &[u8]) -> u8 {
    let mut decimal = 0;
    for (exp, bit) in binary.iter().rev().enumerate() { decimal += bit << exp; }

    decimal
}

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

    pub fn binary_data(&mut self) -> &mut Encoder {
        use std::mem::swap;
        use crate::encoder::qrcode_info::remainder_bits;

        {
            let mut data = vec![];
            swap(&mut self.data, &mut data);

            for decimal in data.into_iter() { push_binary(&mut self.data, 8, decimal as u16); }
        }

        for _ in 0..remainder_bits(self.version) { self.data.push(0); }

        self
    }
}