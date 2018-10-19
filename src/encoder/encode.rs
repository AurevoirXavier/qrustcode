use super::{
    bits::binary,
    mode::Mode,
    Encoder,
};

impl Encoder {
    fn mode_detect(&mut self, message: &str) {
        use super::mode::Mode::*;

        if self.mode != Unknown { return; }

        // reverse the order of Mode, we can pop() the most suitable mode at the end of the loop
        // modes[2] -> Byte mode(ISO 8859-1)
        // Byte mode (UTF-8) should be modes[0], but ignore it
        let mut modes = vec![Chinese, Kanji, Byte, Alphanumeric, Numeric];

        // check every char
        for c in message.chars() {
            let mut fix = 0;
            // check every mode(remained) except Byte(UTF-8)
            for i in 0..modes.len() {
                let i = i - fix;
                if modes[i].not_support(c) {
                    modes.remove(i);

                    //    when fix == mode.len() - 1
                    // -> modes is empty
                    // -> use Byte(UTF-8) mode
                    if fix == 4 {
                        self.mode = Byte;
                        return;
                    }

                    fix += 1;
                }
            }
        }

        self.mode = modes.pop().unwrap();
    }

    fn version_detect(&mut self, len: u16) -> usize {
        use super::qrcode_info::INDICATORS;

        if self.version == 255 {
            use super::qrcode_info::CAPACITIES;

            let mut total_bits = 4 + match self.mode {
                Mode::Numeric => {
                    10 * (len / 3) + match len % 3 {
                        0 => 0,
                        1 => 4,
                        2 => 7,
                        _ => panic!()
                    }
                }
                Mode::Alphanumeric => 11 * (len >> 1) + match len & 1 {
                    0 => 0,
                    1 => 6,
                    _ => panic!()
                },
                Mode::Byte => 8 * len,
                Mode::Kanji | Mode::Chinese => 13 * len,
                _ => panic!() // TODO
            };

            for (&(start, end), indicators) in [(0usize, 8usize), (9, 25), (26, 39)].iter().zip(INDICATORS.iter()) {
                total_bits += indicators[self.mode.to_usize()] as u16;

                if total_bits < CAPACITIES[end][self.ec_level] {
                    for version in start..=end {
                        if total_bits < CAPACITIES[version][self.ec_level] {
                            self.version = version;
                            return indicators[self.mode.to_usize()] as usize;
                        }
                    }
                }
            }

            panic!()
        }

        INDICATORS[match self.version {
            0...8 => 0,
            9...25 => 1,
            26...39 => 2,
            _ => panic!()
        }][self.mode.to_usize()] as usize
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
                    alphanumeric_table(message[i]) as u16
                    +
                    alphanumeric_table(message[i + 1]) as u16,
            ));
        }

        if len & 1 == 1 { data.push(binary(6, alphanumeric_table(*message.last().unwrap()) as u16)); }

        self.data = data.concat();

        self
    }

    fn byte_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder {
        self.data = message.as_bytes()
            .into_iter()
            .map(|&byte| binary(8, byte as u16))
            .flatten()
            .collect();

        self
    }

    fn kanji_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder { self }

    fn chinese_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder { self }

    pub fn encode(&mut self, message: &str) {
        self.mode_detect(message);
        let bits_count = self.version_detect(message.len() as u16);

        match self.mode {
            Mode::Numeric => self.numeric_encode(bits_count, message),
            Mode::Alphanumeric => self.alphanumeric_encode(bits_count, message),
            Mode::Byte => self.byte_encode(bits_count, message),
            Mode::Kanji => self.kanji_encode(bits_count, message),     // TODO
            Mode::Chinese => self.chinese_encode(bits_count, message), // TODO
            _ => panic!()
        }
            .decimal_data()
            .error_correction();

//        println!("{}", self.version);
//        println!("{:?}", self.mode);
    }
}