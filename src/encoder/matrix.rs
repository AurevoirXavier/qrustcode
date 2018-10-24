pub struct Matrix {
    bits: Vec<u8>,
    matrix: Vec<Vec<bool>>,
}

impl Matrix {
    pub fn new(data: Vec<u8>, version: usize) -> Matrix {
        Matrix {
            bits: data,
            matrix: {
                let size = version * 4 + 17;
                let mut matrix = vec![];
                let mut row = vec![];

                row.resize(size, false);
                matrix.resize(size, row);

                matrix
            },
        }
    }
}