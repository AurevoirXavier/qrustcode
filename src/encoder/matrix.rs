fn add_finder_patterns(matrix: &mut Vec<Vec<bool>>) {
    let top_left_corner = matrix.len() - 7;

    for (i, j) in [(0, 0), (top_left_corner, 0), (0, top_left_corner)].iter() {
        for y in 0..7 {
            for x in 0..7 {
                matrix[x + i][y + j] = match x {
                    1 | 5 => match y {
                        1...5 => continue,
                        _ => true
                    }
                    2...4 => match y {
                        1 | 5 => continue,
                        _ => true
                    }
                    _ => true
                };
            }
        }
    }
}


#[test]
fn test() {
    let mut matrix = {
        let mut matrix = vec![];
        let mut row = vec![];

        row.resize(21, false);
        matrix.resize(21, row);

        matrix
    };

    add_finder_patterns(&mut matrix);

    assert_eq!(
        matrix.into_iter()
            .map(|row| row.into_iter()
                .map(|flag| if flag { '■' } else { ' ' })
                .collect::<String>()
            ).collect::<Vec<String>>()
            .join("\n"),
        [
            "\
            ■■■■■■■       ■■■■■■■\n\
            ■     ■       ■     ■\n\
            ■ ■■■ ■       ■ ■■■ ■\n\
            ■ ■■■ ■       ■ ■■■ ■\n\
            ■ ■■■ ■       ■ ■■■ ■\n\
            ■     ■       ■     ■\n\
            ■■■■■■■       ■■■■■■■\n\
            ",
            "                     \n".repeat(7).as_str(),
            "\
            ■■■■■■■              \n\
            ■     ■              \n\
            ■ ■■■ ■              \n\
            ■ ■■■ ■              \n\
            ■ ■■■ ■              \n\
            ■     ■              \n\
            ■■■■■■■              \
            "
        ].join("")
            .to_string()
    )
}

pub struct Matrix<'a> {
    bits: &'a Vec<u8>,
    matrix: Vec<Vec<bool>>,
}

impl<'a> Matrix<'a> {
    fn build_matrix(&mut self) {
        add_finder_patterns(&mut self.matrix);
    }

    pub fn new(data: &Vec<u8>, version: usize) -> Matrix {
        let mut matrix = Matrix {
            bits: data,
            matrix: {
                let size = version * 4 + 17;
                let mut matrix = vec![];
                let mut row = vec![];

                row.resize(size, false);
                matrix.resize(size, row);

                matrix
            },
        };

        matrix.build_matrix();

        matrix
    }
}