// state: u8
// 0 -> 0
// 1 -> 1
// 2 -> unused
pub struct Matrix(Vec<Vec<u8>>);

impl Matrix {
    fn add_finder_patterns(&mut self) -> &mut Matrix {
        let Matrix(matrix) = self;
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

        self
    }

    fn add_separators(&mut self) -> &mut Matrix {
        let Matrix(matrix) = self;
        let len = matrix.len();
        let fix = matrix.len() - 8;

        for i in 0..8 {
            // top left
            // horizontal
            matrix[7][i] = 0;
            // vertical
            matrix[i][7] = 0;

            // vertical
            matrix[i][fix] = 0;

            // bottom left
            // horizontal
            matrix[fix][i] = 0;
        }

        for i in fix..len {
            // top right
            // horizontal
            matrix[7][i] = 0;

            // bottom left
            // vertical
            matrix[i][7] = 0;
        }

        self
    }

    fn add_alignment_patterns(&mut self, version: usize) -> &mut Matrix {
        use crate::encoder::qrcode_info::ALIGNMENT_PATTERN_LOCATIONS;

        if version == 1 { return self; }

        // index from 0 and only 39 versions in array -> version - 2
        let locations = ALIGNMENT_PATTERN_LOCATIONS[version - 2];
        let Matrix(matrix) = self;

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

        self
    }

    fn add_timing_patterns(&mut self) -> &mut Matrix {
        let Matrix(matrix) = self;
        let fix = matrix.len() - 8;
        let mut timing_pattern = [1, 1, 0, 0].iter().cycle();

        for i in 8..fix {
            // horizontal
            matrix[6][i] = *timing_pattern.next().unwrap();

            // vertical
            matrix[i][6] = *timing_pattern.next().unwrap();
        };

        self
    }

    pub fn new(data: &Vec<u8>, version: usize) -> Matrix {
        let mut matrix = Matrix({
            let size = version * 4 + 17;
            let mut matrix = vec![];
            let mut row = vec![];

            row.resize(size, 2);
            matrix.resize(size, row);

            matrix
        });

        matrix
            .add_finder_patterns()
            .add_separators()
            .add_alignment_patterns(version)
            .add_timing_patterns();

        matrix
    }
}

#[test]
fn test_add_finder_patterns() {
    let mut matrix = Matrix({
        let mut matrix = vec![];
        let mut row = vec![];

        row.resize(21, 2);
        matrix.resize(21, row);

        matrix
    });

    matrix.add_finder_patterns();

    assert_eq!(
        matrix.0.iter()
            .map(|row| row.iter()
                .map(|state| match state {
                    0 => '□',
                    1 => '■',
                    _ => ' '
                }).collect::<String>()
            ).collect::<Vec<String>>()
            .join("\n"),
        [
            "\
            ■■■■■■■       ■■■■■■■\n\
            ■□□□□□■       ■□□□□□■\n\
            ■□■■■□■       ■□■■■□■\n\
            ■□■■■□■       ■□■■■□■\n\
            ■□■■■□■       ■□■■■□■\n\
            ■□□□□□■       ■□□□□□■\n\
            ■■■■■■■       ■■■■■■■\n",
            "                     \n".repeat(7).as_str(),
            "■■■■■■■              \n\
            ■□□□□□■              \n\
            ■□■■■□■              \n\
            ■□■■■□■              \n\
            ■□■■■□■              \n\
            ■□□□□□■              \n\
            ■■■■■■■              "
        ].join("")
            .to_string()
    );
}

#[test]
fn test_add_separators() {
    let mut matrix = Matrix({
        let mut matrix = vec![];
        let mut row = vec![];

        row.resize(21, 2);
        matrix.resize(21, row);

        matrix
    });

    matrix
        .add_finder_patterns()
        .add_separators();

    assert_eq!(
        matrix.0.iter()
            .map(|row| row.iter()
                .map(|state|  match state {
                    0 => '□',
                    1 => '■',
                    _ => ' '
                }).collect::<String>()
            ).collect::<Vec<String>>()
            .join("\n"),
        [
            "\
            ■■■■■■■□     □■■■■■■■\n\
            ■□□□□□■□     □■□□□□□■\n\
            ■□■■■□■□     □■□■■■□■\n\
            ■□■■■□■□     □■□■■■□■\n\
            ■□■■■□■□     □■□■■■□■\n\
            ■□□□□□■□     □■□□□□□■\n\
            ■■■■■■■□     □■■■■■■■\n\
            □□□□□□□□     □□□□□□□□\n",
            "                     \n".repeat(5).as_str(),
            "□□□□□□□□             \n\
            ■■■■■■■□             \n\
            ■□□□□□■□             \n\
            ■□■■■□■□             \n\
            ■□■■■□■□             \n\
            ■□■■■□■□             \n\
            ■□□□□□■□             \n\
            ■■■■■■■□             "
        ].join("")
            .to_string()
    );
}

#[test]
fn test_add_alignment_patterns() {
    let mut matrix = Matrix({
        let mut matrix = vec![];
        let mut row = vec![];

        row.resize(25, 2);
        matrix.resize(25, row);

        matrix
    });

    matrix
        .add_finder_patterns()
        .add_separators()
        .add_alignment_patterns(2);

    assert_eq!(
        matrix.0.iter()
            .map(|row| row.iter()
                .map(|state|  match state {
                    0 => '□',
                    1 => '■',
                    _ => ' '
                }).collect::<String>()
            ).collect::<Vec<String>>()
            .join("\n"),
        [
            "\
            ■■■■■■■□         □■■■■■■■\n\
            ■□□□□□■□         □■□□□□□■\n\
            ■□■■■□■□         □■□■■■□■\n\
            ■□■■■□■□         □■□■■■□■\n\
            ■□■■■□■□         □■□■■■□■\n\
            ■□□□□□■□         □■□□□□□■\n\
            ■■■■■■■□         □■■■■■■■\n\
            □□□□□□□□         □□□□□□□□\n",
            "                         \n".repeat(8).as_str(),
            "                ■■■■■    \n",
            "□□□□□□□□        ■□□□■    \n",
            "■■■■■■■□        ■□■□■    \n\
            ■□□□□□■□        ■□□□■    \n\
            ■□■■■□■□        ■■■■■    \n\
            ■□■■■□■□                 \n\
            ■□■■■□■□                 \n\
            ■□□□□□■□                 \n\
            ■■■■■■■□                 "
        ].join("")
            .to_string()
    );
}

#[test]
fn test_add_timing_patterns() {
    let mut matrix = Matrix({
        let mut matrix = vec![];
        let mut row = vec![];

        row.resize(25, 2);
        matrix.resize(25, row);

        matrix
    });

    matrix
        .add_finder_patterns()
        .add_separators()
        .add_alignment_patterns(2)
        .add_timing_patterns();

    assert_eq!(
        matrix.0.iter()
            .map(|row| row.iter()
                .map(|state|  match state {
                    0 => '□',
                    1 => '■',
                    _ => ' '
                }).collect::<String>()
            ).collect::<Vec<String>>()
            .join("\n"),
        [
            "\
            ■■■■■■■□         □■■■■■■■\n\
            ■□□□□□■□         □■□□□□□■\n\
            ■□■■■□■□         □■□■■■□■\n\
            ■□■■■□■□         □■□■■■□■\n\
            ■□■■■□■□         □■□■■■□■\n\
            ■□□□□□■□         □■□□□□□■\n\
            ■■■■■■■□■□■□■□■□■□■■■■■■■\n\
            □□□□□□□□         □□□□□□□□\n",
            "      ■                  \n",
            "      □                  \n",
            "      ■                  \n",
            "      □                  \n",
            "      ■                  \n",
            "      □                  \n",
            "      ■                  \n",
            "      □                  \n",
            "      ■         ■■■■■    \n",
            "□□□□□□□□        ■□□□■    \n",
            "■■■■■■■□        ■□■□■    \n\
            ■□□□□□■□        ■□□□■    \n\
            ■□■■■□■□        ■■■■■    \n\
            ■□■■■□■□                 \n\
            ■□■■■□■□                 \n\
            ■□□□□□■□                 \n\
            ■■■■■■■□                 "
        ].join("")
            .to_string()
    );
}