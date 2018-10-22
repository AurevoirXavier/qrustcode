use crate::encoder::{
    Encoder,
    mode::Mode::*,
};

impl Encoder {
    pub fn mode_detect(&mut self, message: &str) {
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

    pub fn version_detect(&mut self, len: u16) -> usize {
        use crate::encoder::qrcode_info::INDICATORS;

        if self.version == 255 {
            use crate::encoder::qrcode_info::CAPACITIES;

            let mut total_bits = 4 + match self.mode {
                Numeric => {
                    10 * (len / 3) + match len % 3 {
                        0 => 0,
                        1 => 4,
                        2 => 7,
                        _ => panic!()
                    }
                }
                Alphanumeric => 11 * (len >> 1) + match len & 1 {
                    0 => 0,
                    1 => 6,
                    _ => panic!()
                },
                Byte => 8 * len,
                Kanji | Chinese => 13 * len,
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
}
