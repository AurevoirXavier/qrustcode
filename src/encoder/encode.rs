use super::{
    Encoder,
    bits::push_binary,
};

impl Encoder {
    fn numeric_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder {
        let len = message.len();
        let edge = len / 3 * 3;

        self.data = vec![0, 0, 0, 1];
        push_binary(&mut self.data, bits_count, len as u16);

        for i in (0..edge).step_by(3) { push_binary(&mut self.data, 10, message[i..i + 3].parse().unwrap()); }
        match len - edge {
            bits @ 1...2 => push_binary(&mut self.data, 1 + 3 * bits, message[edge..len].parse().unwrap()),
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
        push_binary(&mut self.data, bits_count, len as u16);

        for i in (0..len >> 1 << 1).step_by(2) {
            push_binary(
                &mut self.data,
                11,
                45 *
                    alphanumeric_table(message[i]) as u16
                    +
                    alphanumeric_table(message[i + 1]) as u16,
            );
        }
        if len & 1 == 1 { push_binary(&mut self.data, 6, alphanumeric_table(message[len]) as u16); }

        self
    }

    fn byte_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder {
        self.data = vec![0, 1, 0, 0];
        push_binary(&mut self.data, bits_count, message.len() as u16);

        for byte in message.as_bytes() { push_binary(&mut self.data, 8, *byte as u16); }

        self
    }

    fn kanji_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder {
        use encoding_rs::SHIFT_JIS;

        self.data = vec![1, 0, 0, 0];
        push_binary(&mut self.data, bits_count, message.len() as u16);

        let (message, _, _) = SHIFT_JIS.encode(message);
        for kanji in message.chunks(2) {
            // "茗" -- Shift JIS value --> 0xe4aa
            // 0xe4aa - 0xc140 = 0x236a => {
            //     0x236a >> 8 = 0x23
            //     0x236a & 0xff = 0x6a
            // } => (0x23 * 0xc0 = 0x1a40) + 0x6a = 0x1aaa = 0b1101010101010
            let shift_jis_value = kanji[0] as u16 * 256 + kanji[1] as u16;
            let decimal = if shift_jis_value < 0xe040 { shift_jis_value - 0x8140 } else { shift_jis_value - 0xc140 };

            push_binary(&mut self.data, 13, (decimal >> 8) * 0xc0 + (decimal & 0xff));
        }

        self
    }

//    fn chinese_encode(&mut self, bits_count: usize, message: &str) -> &mut Encoder { self } // TODO

    pub fn encode(&mut self, message: &str) {
        use super::mode::Mode::*;

        self.mode_detect(message);
        let bits_count = self.version_detect(message.len() as u16);

        match self.mode {
            Numeric => self.numeric_encode(bits_count, message),
            Alphanumeric => self.alphanumeric_encode(bits_count, message),
            Byte => self.byte_encode(bits_count, message),
            Kanji => self.kanji_encode(bits_count, message),
//            Chinese => self.chinese_encode(bits_count, message), // TODO
            _ => panic!()
        }
            .decimal_data()
            .interleave_with_ec()
            .binary_data();

//        println!("{}", self.version);
//        println!("{:?}", self.mode);
    }
}