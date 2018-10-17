use super::{
    bits::binary,
    Encoder,
};

impl Encoder {
    fn mode_detect(&mut self, message: &str) {
        if self.mode != 255 { return; }

        fn numeric(c: char) -> bool {
            match c {
                '0'...'9' => true,
                _ => false
            }
        }

        fn chinese(c: char) -> bool { if c >= '\u{4e00}' && c <= '\u{9fff}' { true } else { false } }

        let mut modes = vec![0, 1, 3, 4];
        let mut fns = [numeric, numeric, chinese, numeric, chinese];

        // check every char
        for c in message.chars() {
            let mut fix = 0;
            // check every mode(remained)
            for i in 0..modes.len() {
                let i = i - fix;

                if !fns[modes[i]](c) {
                    modes.remove(i);

                    //    when fix == mode.len() - 1
                    // -> modes is empty
                    // -> use Byte(UTF-8) mode
                    if fix == 3 {
                        self.mode = 2;
                        return;
                    }

                    fix += 1;
                }
            }
        }

        self.mode = modes[0] as u8;
    }

    fn version_detect(&mut self, len: u16) -> usize {
        use super::qrcode_info::INDICATORS;

        if self.version == 255 {
            use super::qrcode_info::CAPACITIES;

            let mut total_bits = 4 + match self.mode {
                0 => {
                    10 * (len / 3) + match len % 3 {
                        0 => 0,
                        1 => 4,
                        2 => 7,
                        _ => panic!()
                    }
                }
                1 => 11 * (len >> 1) + match len & 1 {
                    0 => 0,
                    1 => 6,
                    _ => panic!()
                },
                2 => 8 * len,
                3 | 4 => 13 * len,
                _ => panic!() // TODO
            };

            for (&(start, end), indicators) in [(0usize, 8usize), (9, 25), (26, 39)].iter().zip(INDICATORS.iter()) {
                total_bits += indicators[self.mode as usize] as u16;

                if total_bits < CAPACITIES[end][self.ec_level] {
                    for version in start..=end {
                        if total_bits < CAPACITIES[version][self.ec_level] {
                            self.version = version;
                            return indicators[self.mode as usize] as usize;
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

        self.mode_detect(message);
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
//        println!("{}", self.mode);
    }
}