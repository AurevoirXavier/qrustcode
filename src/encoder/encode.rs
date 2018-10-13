use super::{
    bits::binary, qrcode_info::{
        INDICATORS,
        alphanumeric_table,
    },
    Encoder,
};

impl Encoder {
    fn numeric_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder {
        let len = message.len();
        let edge = len / 3 * 3;
        let mut data = vec![vec![0, 0, 0, 1]];

        data.push(binary(bits_count, len as u16));

        for i in (0..edge).step_by(3) { data.push(binary(bits_count, message[i..i + 3].parse().unwrap())); }

        match len - edge {
            bits @ 1...2 => data.push(binary(1 + 3 * bits, message[edge..len].parse().unwrap())),
            0 => (),
            _ => panic!()
        }

        self.data = data.concat();

        self
    }

    fn alphanumeric_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder {
        let message = message.as_bytes();
        let len = message.len();
        let mut data = vec![vec![0, 0, 1, 0]];

        data.push(binary(bits_count, len as u16));

        for i in (0..len >> 1 << 1).step_by(2) { data.push(binary(11, 45 * alphanumeric_table(message[i]) + alphanumeric_table(message[i + 1]))); }

        if len & 1 == 1 { data.push(binary(6, alphanumeric_table(*message.last().unwrap()))); }

        self.data = data.concat();

        self
    }

    fn byte_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder {
        self
    }

    fn kanji_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder {
        self
    }

    fn chinese_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder {
        self
    }

    pub fn encode(&mut self, message: &str) {
        let bits_counts = INDICATORS[match self.version {
            0...8 => 0,
            9...25 => 1,
            26...39 => 2,
            _ => panic!()
        }];

        match self.mode {
            0 => self.numeric_encode(bits_counts[0] as usize, message),
            1 => self.alphanumeric_encode(bits_counts[1] as usize, message),
            2 => self.byte_encode(bits_counts[2] as usize, message),    // TODO
            3 => self.kanji_encode(bits_counts[3] as usize, message),   // TODO
            4 => self.chinese_encode(bits_counts[3] as usize, message), // TODO
            _ => panic!()
        }
            .decimal_data()
            .error_correction();

//        println!("{:?}", self.data);
//        println!("{:?}", self.ec_data);
    }
}