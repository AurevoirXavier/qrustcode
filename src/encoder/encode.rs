use super::{
    bits::binary,
    mode::Mode,
    Encoder,
};

impl Encoder {
    fn mode_detect(&mut self, message: &str) {
        use super::mode::Mode::*;

        if let Unknown = self.mode { () } else { return; }

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
                    fix += 1;

                    // use Byte(UTF-8) mode
                    if fix == modes.len() {
                        self.mode = Byte;
                        return;
                    }
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

        self.data = vec![0, 0, 0, 1];
        self.data.extend_from_slice(binary(bits_count, len as u16).as_slice());

        for i in (0..edge).step_by(3) { self.data.extend_from_slice(binary(10, message[i..i + 3].parse().unwrap()).as_slice()); }
        match len - edge {
            bits @ 1...2 => self.data.extend_from_slice(binary(1 + 3 * bits, message[edge..len].parse().unwrap()).as_slice()),
            0 => (),
            _ => panic!()
        }

        self
    }

    fn alphanumeric_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder {
        use super::qrcode_info::alphanumeric_table;

        let message = message.as_bytes();
        let len = message.len();

        self.data = vec![0, 0, 1, 0];
        self.data.extend_from_slice(binary(bits_count, len as u16).as_slice());

        for i in (0..len >> 1 << 1).step_by(2) {
            self.data.extend_from_slice(binary(
                11,
                45 *
                    alphanumeric_table(message[i]) as u16
                    +
                    alphanumeric_table(message[i + 1]) as u16,
            ).as_slice());
        }
        if len & 1 == 1 { self.data.extend_from_slice(binary(6, alphanumeric_table(*message.last().unwrap()) as u16).as_slice()); }

        self
    }

    fn byte_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder {
        self.data = vec![0, 1, 0, 0];
        self.data.extend_from_slice(binary(bits_count, message.len() as u16).as_slice());

        for &byte in message.as_bytes() { self.data.extend_from_slice(binary(8, byte as u16).as_slice()); }

        self
    }

    fn kanji_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder {
        use encoding_rs::SHIFT_JIS;

        self.data = vec![1, 0, 0, 0];
        self.data.extend_from_slice(binary(bits_count, message.len() as u16).as_slice());

        let (message, _, _) = SHIFT_JIS.encode(message);
        for kanji in message.chunks(2) {
            // "茗" -- Shift JIS value --> 0xe4aa
            // 0xe4aa - 0xc140 = 0x236a => {
            //     0x236a >> 8 = 0x23
            //     0x236a & 0xff = 0x6a
            // } => (0x23 * 0xc0 = 0x1a40) + 0x6a = 0x1aaa = 0b1101010101010
            let shift_jis_value = kanji[0] as u16 * 256 + kanji[1] as u16;
            let decimal = if shift_jis_value < 0xe040 { shift_jis_value - 0x8140 } else { shift_jis_value - 0xc140 };
            self.data.extend_from_slice(binary(13, (decimal >> 8) * 0xc0 + (decimal & 0xff)).as_slice());
        }

        self
    }

    fn chinese_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder { self } // TODO

    pub fn encode(&mut self, message: &str) {
        self.mode_detect(message);
        let bits_count = self.version_detect(message.len() as u16);

        match self.mode {
            Mode::Numeric => self.numeric_encode(bits_count, message),
            Mode::Alphanumeric => self.alphanumeric_encode(bits_count, message),
            Mode::Byte => self.byte_encode(bits_count, message),
            Mode::Kanji => self.kanji_encode(bits_count, message),
            Mode::Chinese => self.chinese_encode(bits_count, message), // TODO
            _ => panic!()
        }
            .decimal_data()
            .error_correction();

//        println!("{}", self.version);
//        println!("{:?}", self.mode);
    }
}