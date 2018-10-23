use crate::encoder::Encoder;

fn gen_indexes(from: &mut usize, step: usize, count: u8) -> Vec<usize> {
    let mut indexes = vec![];

    for _ in 0..count {
        indexes.push(*from);
        *from += step;
    }

    indexes
}

#[test]
fn test() {
    let g1_blocks_num = 2;
    let g2_blocks_num = 2;
    let g1_cw_per_block = 15;
    let g2_cw_per_block = 16;

    let mut indexes = {
        let mut i = 0;
        let mut indexes = gen_indexes(&mut i, g1_cw_per_block, g1_blocks_num);

        indexes.extend_from_slice(gen_indexes(
            &mut i,
            g2_cw_per_block,
            g2_blocks_num,
        ).as_slice());

        indexes
    };

    assert_eq!(indexes, vec![0, 15, 30, 46]);
}

impl Encoder {
    fn interleave(&mut self) -> &mut Encoder {
        use std::thread;
        use super::error_correct::error_correct;
        use crate::encoder::qrcode_info::{DATA_DISTRIBUTIONS, EC_CW_PER_BLOCKS};

        let ec_cw_per_blocks = EC_CW_PER_BLOCKS[self.version][self.ec_level];
        let data_distribution = DATA_DISTRIBUTIONS[self.version][self.ec_level];

        let g1_blocks_num = data_distribution[0];
        let g2_blocks_num = data_distribution[2];

        if g1_blocks_num + g2_blocks_num == 1 {
            self.data.extend_from_slice(error_correct(
                self.data.clone(),
                ec_cw_per_blocks
            ).as_slice());

            return self;
        }

        let g1_cw_per_block = data_distribution[1] as usize;
        let g2_cw_per_block = g1_cw_per_block + 1;

        let data = self.data.clone();
        let final_data = thread::spawn(move || {
            let mut final_data = vec![];
            let mut indexes = {
                let mut i = 0;
                let mut indexes = gen_indexes(&mut i, g1_cw_per_block, g1_blocks_num);

                indexes.extend_from_slice(gen_indexes(
                    &mut i,
                    g2_cw_per_block,
                    g2_blocks_num,
                ).as_slice());

                indexes
            };

            for _ in 0..g1_blocks_num {
                for i in indexes.iter_mut() {
                    final_data.push(data[*i]);
                    *i += 1;
                }
            }

            // g2_cw_per_block is always `1` greater than g1_cw_per_block
            for i in indexes[g1_blocks_num as usize..].iter() { final_data.push(data[*i]); }

            final_data
        });

        let mut final_ec_data = vec![];
        {
            let mut ec_data = vec![];
            {
                let g1_cw = g1_blocks_num as usize * g1_cw_per_block;
                for chunk in self.data[..g1_cw].chunks(g1_cw_per_block) {
                    ec_data.extend_from_slice(error_correct(
                        chunk.to_vec(),
                        ec_cw_per_blocks,
                    ).as_slice())
                }

                for chunk in self.data[g1_cw..].chunks(g2_cw_per_block) {
                    ec_data.extend_from_slice(error_correct(
                        chunk.to_vec(),
                        ec_cw_per_blocks,
                    ).as_slice())
                }
            }

            let mut indexes = gen_indexes(
                &mut 0,
                ec_cw_per_blocks as usize,
                g1_blocks_num + g2_blocks_num,
            );

            for _ in 0..ec_cw_per_blocks {
                for i in indexes.iter_mut() {
                    final_ec_data.push(ec_data[*i]);
                    *i += 1;
                }
            }
        }

        self.data = final_data.join().unwrap();
        self.data.extend_from_slice(final_ec_data.as_slice());

        self
    }

    pub fn final_structure(&mut self) {
        self.interleave();
    }
}
