use crate::encoder::Encoder;

type GroupWithEC = Vec<(Vec<u8>, Vec<u8>)>;

fn build_group(data: &[u8], chunk_size: usize, ec_cw_per_block: u8) -> GroupWithEC {
    use super::error_correct::error_correct;

    let mut group = vec![];

    for chunk in data.chunks(chunk_size) {
        let data = chunk.to_vec();
        group.push((
            data.clone(),
            error_correct(data, ec_cw_per_block)
        ));
    }

    group
}

fn groups(data: &Vec<u8>, ec_cw_per_block: u8, groups: [u8; 4]) -> [GroupWithEC; 2] {
    unimplemented!()
}

fn interleave(groups: [GroupWithEC; 2], min_len: usize) -> Vec<u8> {
    use std::thread;

    let mut final_data = vec![];
    let mut final_ec_data = vec![];

    for (block, ec_data) in groups {
        final_data.push(block[i]);
        final_ec_data.push(ec_data[i]);
    }

    final_data.extend_from_slice(final_ec_data.as_slice());

    final_data
}

impl Encoder {
    pub fn final_structure(&mut self) {
        use crate::encoder::qrcode_info::{DATA_DISTRIBUTIONS, EC_CW_PER_BLOCKS};
        let final_data = interleave(
            groups(
                &self.data,
                EC_CW_PER_BLOCKS[self.version][self.ec_level],
                DATA_DISTRIBUTIONS[self.version][self.ec_level],
            ),
            data_distribution[1] as usize,
        );
    }
}