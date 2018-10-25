fn add_finder_patterns(matrix: &mut Vec<Vec<u8>>) {
    let top_left_corner = matrix.len() - 7;

    for (i, j) in [(0, 0), (top_left_corner, 0), (0, top_left_corner)].iter() {
        for y in 0..7 {
            for x in 0..7 {
                matrix[x + i][y + j] = match x {
                    1 | 5 => match y {
                        1...5 => 0,
                        _ => 1
                    }
                    2...4 => match y {
                        1 | 5 => 0,
                        _ => 1
                    }
                    _ => 1
                };
            }
        }
    }
}

#[test]
fn test_add_finder_patterns() {
    let mut matrix = {
        let mut matrix = vec![];
        let mut row = vec![];

        row.resize(21, 2);
        matrix.resize(21, row);

        matrix
    };

    add_finder_patterns(&mut matrix);

    assert_eq!(
        matrix.into_iter()
            .map(|row| row.into_iter()
                .map(|state| if state == 1 { '■' } else { ' ' })
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

fn add_alignment_patterns(matrix: &mut Vec<Vec<u8>>, version: usize) {
    use crate::encoder::qrcode_info::ALIGNMENT_PATTERN_LOCATIONS;

    if version == 1 { return; }

    // index from 0 and only 39 versions in array -> version - 2
    let locations = ALIGNMENT_PATTERN_LOCATIONS[version - 2];

    for &y in locations.iter() {
        for &x in locations.iter() {
            if matrix[x as usize][y as usize] == 1 { continue; }

            // get top left module coordinate
            let (i, j) = (x as usize - 2, y as usize - 2);

            for y in 0..5 {
                for x in 0..5 {
                    matrix[x + i][y + j] = match x {
                        1...3 => match y {
                            1...3 => 0,
                            _ => 1
                        }
                        _ => 1
                    }
                }
            }

            // center module
            matrix[x as usize][y as usize] = 1;
        }
    }
}

#[test]
fn test_add_alignment_patterns() {
    let mut matrix = {
        let mut matrix = vec![];
        let mut row = vec![];

        row.resize(25, 2);
        matrix.resize(25, row);

        matrix
    };

    add_finder_patterns(&mut matrix);
    add_alignment_patterns(&mut matrix, 2);

    assert_eq!(
        matrix.into_iter()
            .map(|row| row.into_iter()
                .map(|state| if state == 1 { '■' } else { ' ' })
                .collect::<String>()
            ).collect::<Vec<String>>()
            .join("\n"),
        [
            "\
            ■■■■■■■           ■■■■■■■\n\
            ■     ■           ■     ■\n\
            ■ ■■■ ■           ■ ■■■ ■\n\
            ■ ■■■ ■           ■ ■■■ ■\n\
            ■ ■■■ ■           ■ ■■■ ■\n\
            ■     ■           ■     ■\n\
            ■■■■■■■           ■■■■■■■\n",
            "                         \n".repeat(9).as_str(),
            "                ■■■■■    \n",
            "                ■   ■    \n",
            "■■■■■■■         ■ ■ ■    \n\
            ■     ■         ■   ■    \n\
            ■ ■■■ ■         ■■■■■    \n\
            ■ ■■■ ■                  \n\
            ■ ■■■ ■                  \n\
            ■     ■                  \n\
            ■■■■■■■                  \
            "
        ].join("")
            .to_string()
    )
}

pub struct Matrix<'a> {
    bits: &'a Vec<u8>,

    // state: u8
    // 0 -> 0
    // 1 -> 1
    // 2 -> unused
    matrix: Vec<Vec<u8>>,
}

impl<'a> Matrix<'a> {
    fn build_matrix(&mut self, version: usize) {
        add_finder_patterns(&mut self.matrix);
        add_alignment_patterns(&mut self.matrix, version);
    }

    pub fn new(data: &Vec<u8>, version: usize) -> Matrix {
        let mut matrix = Matrix {
            bits: data,
            matrix: {
                let size = version * 4 + 17;
                let mut matrix = vec![];
                let mut row = vec![];

                row.resize(size, 2);
                matrix.resize(size, row);

                matrix
            },
        };

        matrix.build_matrix(version);

        matrix
    }
}