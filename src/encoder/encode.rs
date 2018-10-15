use super::{
    bits::binary,
    Encoder,
};

impl Encoder {
    fn mode_detect(&mut self) {}

    fn version_detect(&mut self, len: u16) -> usize {
        use super::qrcode_info::INDICATORS;

        if self.version == 255 {
            use super::qrcode_info::CAPACITIES;

            for (&version, indicators) in [8usize, 25, 39].iter().zip(INDICATORS.iter()) {
                let capacity = 4 + match self.mode {
                    0 => {
                        indicators[0] as u16 + 10 * (len / 3) + match len % 3 {
                            0 => 0,
                            1 => 4,
                            2 => 7,
                            _ => panic!()
                        }
                    }
                    1 => indicators[1] as u16 + 11 * (len >> 2) + match len & 1 {
                        0 => 0,
                        1 => 6,
                        _ => panic!()
                    },
                    2 => indicators[2] as u16 + 8 * len,
                    3 | 4 => indicators[3] as u16 + 13 * len,
                    _ => panic!() // TODO
                };

                if capacity < CAPACITIES[version][self.ec_level] {
                    self.version = 0;

                    return indicators[self.mode as usize] as usize;
                }
            }
            panic!()
        }

        INDICATORS[match self.version {
            0...8 => 0,
            9...25 => 1,
            26...39 => 2,
            _ => panic!()
        }][self.mode as usize] as usize
    }

    fn numeric_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder {
        let len = message.len();
        let edge = len / 3 * 3;
        let mut data = vec![vec![0, 0, 0, 1]];

        data.push(binary(bits_count, len as u16));

        for i in (0..edge).step_by(3) { data.push(binary(10, message[i..i + 3].parse().unwrap())); }

        match len - edge {
            bits @ 1...2 => data.push(binary(1 + 3 * bits, message[edge..len].parse().unwrap())),
            0 => (),
            _ => panic!()
        }

        self.data = data.concat();

        self
    }

    fn alphanumeric_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder {
        use super::qrcode_info::alphanumeric_table;

        let message = message.as_bytes();
        let len = message.len();
        let mut data = vec![vec![0, 0, 1, 0]];

        data.push(binary(bits_count, len as u16));

        for i in (0..len >> 1 << 1).step_by(2) {
            data.push(binary(
                11,
                45 *
                    alphanumeric_table(message[i])
                    +
                    alphanumeric_table(message[i + 1])));
        }

        if len & 1 == 1 { data.push(binary(6, alphanumeric_table(*message.last().unwrap()))); }

        self.data = data.concat();

        self
    }

    fn byte_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder { self }

    fn kanji_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder { self }

    fn chinese_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder { self }

    pub fn encode(&mut self, message: &str) {
        use super::qrcode_info::INDICATORS;

        let bits_count = self.version_detect(message.len() as u16);

        match self.mode {
            0 => self.numeric_encode(bits_count, message),
            1 => self.alphanumeric_encode(bits_count, message),
            2 => self.byte_encode(bits_count, message),    // TODO
            3 => self.kanji_encode(bits_count, message),   // TODO
            4 => self.chinese_encode(bits_count, message), // TODO
            _ => panic!()
        }
            .decimal_data()
            .error_correction();

        println!("{}", self.version);
    }
}