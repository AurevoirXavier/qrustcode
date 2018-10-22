use crate::encoder::Encoder;

impl Encoder {
    pub fn groups(&mut self) {
        use crate::encoder::qrcode_info::DATA_DISTRIBUTIONS;

        let blocks = DATA_DISTRIBUTIONS[self.version][self.ec_level];
        let (group1, blocks1, groups2, blocks2) = (blocks[0], blocks[1], blocks[2], blocks[3]);

        if blocks1 + blocks2 == 0 {
            // TODO
        }


    }
}