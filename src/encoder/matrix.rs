// state: u8
// 0 -> 0
// 1 -> 1
// 2 -> reserved
// 3 -> unused
pub struct Matrix(Vec<Vec<u8>>);

impl Matrix {
    fn add_finder_patterns(&mut self) -> &mut Matrix {
        let Matrix(matrix) = self;
        let fix = matrix.len() - 7;

        for (i, j) in [(0, 0), (fix, 0), (0, fix)].iter() {
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
            matrix[7][i] = 0; // horizontal
            matrix[i][7] = 0; // vertical
            // top right
            matrix[i][fix] = 0; // vertical
            // bottom left
            matrix[fix][i] = 0; // horizontal
        }

        for i in fix..len {
            // top right
            matrix[7][i] = 0; // horizontal
            // bottom left
            matrix[i][7] = 0; // vertical
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

    fn add_dark_module_and_reserved_areas(&mut self, version: usize) -> &mut Matrix {
        let Matrix(matrix) = self;
        let len = matrix.len();
        let fix = len - 8;

        // reserved areas
        for i in 0..9 {
            // horizontal
            matrix[8][i] = 2;
            // vertical
            matrix[i][8] = 2;
        }

        for i in fix..len {
            // horizontal
            matrix[8][i] = 2;
            // vertical
            matrix[i][8] = 2;
        }

        // avoid timing pattern
        matrix[6][8] = 1;
        matrix[8][6] = 1;
        // avoid dark module
        matrix[fix][8] = 1;

        if version > 6 {
            for i in fix - 3..fix {
                for j in 0..6 {
                    // top-right
                    matrix[i][j] = 2;
                    // bottom-left
                    matrix[j][i] = 2;
                }
            }
        }

        self
    }

    fn place_data(&mut self, data: &Vec<u8>) -> &mut Matrix {
        let Matrix(matrix) = self;
        let len = matrix.len();
        let mut data = data.iter();
        let mut upward = true;

        for x in (0..len).rev().step_by(2) {
            // timing pattern
            if x == 6 { continue; }

            let range = if upward { (0..len).rev() } else { 0..len };
            for y in range {
                if matrix[y][x] == 3 { matrix[y][x] = *data.next().unwrap(); }
                if matrix[y][x - 1] == 3 { matrix[y][x - 1] = *data.next().unwrap()); }
            }

            upward = !upward;
        }

        self
    }

    pub fn new(data: &Vec<u8>, version: usize) -> Matrix {
        let mut matrix = Matrix({
            let size = version * 4 + 17;
            let mut matrix = vec![];
            let mut row = vec![];

            row.resize(size, 3);
            matrix.resize(size, row);

            matrix
        });

        matrix
            .add_finder_patterns()
            .add_separators()
            .add_alignment_patterns(version)
            .add_timing_patterns()
            .add_dark_module_and_reserved_areas(version);

        matrix
    }
}

#[test]
fn test_add_finder_patterns() {
    let mut matrix = Matrix({
        let mut matrix = vec![];
        let mut row = vec![];

        row.resize(21, 3);
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
            "\
            ■■■■■■■              \n\
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

        row.resize(21, 3);
        matrix.resize(21, row);

        matrix
    });

    matrix
        .add_finder_patterns()
        .add_separators();

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
            ■■■■■■■□     □■■■■■■■\n\
            ■□□□□□■□     □■□□□□□■\n\
            ■□■■■□■□     □■□■■■□■\n\
            ■□■■■□■□     □■□■■■□■\n\
            ■□■■■□■□     □■□■■■□■\n\
            ■□□□□□■□     □■□□□□□■\n\
            ■■■■■■■□     □■■■■■■■\n\
            □□□□□□□□     □□□□□□□□\n",
            "                     \n".repeat(5).as_str(),
            "\
            □□□□□□□□             \n\
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

        row.resize(25, 3);
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
                .map(|state| match state {
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
            "                ■■■■■    \n\
            □□□□□□□□        ■□□□■    \n\
            ■■■■■■■□        ■□■□■    \n\
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

        row.resize(25, 3);
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
                .map(|state| match state {
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
            "      ■         ■■■■■    \n\
            □□□□□□□□        ■□□□■    \n\
            ■■■■■■■□        ■□■□■    \n\
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
fn test_add_dark_module_and_reserved_areas() {
    // version 2
    let mut matrix = Matrix({
        let mut matrix = vec![];
        let mut row = vec![];

        row.resize(25, 3);
        matrix.resize(25, row);

        matrix
    });

    matrix
        .add_finder_patterns()
        .add_separators()
        .add_alignment_patterns(2)
        .add_timing_patterns()
        .add_dark_module_and_reserved_areas(2);

    assert_eq!(
        matrix.0.iter()
            .map(|row| row.iter()
                .map(|state| match state {
                    0 => '□',
                    1 => '■',
                    2 => '○',
                    _ => ' '
                }).collect::<String>()
            ).collect::<Vec<String>>()
            .join("\n"),
        [
            "\
            ■■■■■■■□○        □■■■■■■■\n\
            ■□□□□□■□○        □■□□□□□■\n\
            ■□■■■□■□○        □■□■■■□■\n\
            ■□■■■□■□○        □■□■■■□■\n\
            ■□■■■□■□○        □■□■■■□■\n\
            ■□□□□□■□○        □■□□□□□■\n\
            ■■■■■■■□■□■□■□■□■□■■■■■■■\n\
            □□□□□□□□○        □□□□□□□□\n\
            ○○○○○○■○○        ○○○○○○○○\n",
            "      □                  \n",
            "      ■                  \n",
            "      □                  \n",
            "      ■                  \n",
            "      □                  \n",
            "      ■                  \n",
            "      □                  \n",
            "      ■         ■■■■■    \n\
            □□□□□□□□■       ■□□□■    \n\
            ■■■■■■■□○       ■□■□■    \n\
            ■□□□□□■□○       ■□□□■    \n\
            ■□■■■□■□○       ■■■■■    \n\
            ■□■■■□■□○                \n\
            ■□■■■□■□○                \n\
            ■□□□□□■□○                \n\
            ■■■■■■■□○                "
        ].join("")
            .to_string()
    );

    // version 7
    let mut matrix = Matrix({
        let mut matrix = vec![];
        let mut row = vec![];

        row.resize(45, 3);
        matrix.resize(45, row);

        matrix
    });

    matrix
        .add_finder_patterns()
        .add_separators()
        .add_alignment_patterns(7)
        .add_timing_patterns()
        .add_dark_module_and_reserved_areas(7);

    assert_eq!(
        matrix.0.iter()
            .map(|row| row.iter()
                .map(|state| match state {
                    0 => '□',
                    1 => '■',
                    2 => '○',
                    _ => ' '
                }).collect::<String>()
            ).collect::<Vec<String>>()
            .join("\n"),
        [
            "\
            ■■■■■■■□○                         ○○○□■■■■■■■\n\
            ■□□□□□■□○                         ○○○□■□□□□□■\n\
            ■□■■■□■□○                         ○○○□■□■■■□■\n\
            ■□■■■□■□○                         ○○○□■□■■■□■\n\
            ■□■■■□■□○           ■■■■■         ○○○□■□■■■□■\n\
            ■□□□□□■□○           ■□□□■         ○○○□■□□□□□■\n\
            ■■■■■■■□■□■□■□■□■□■□■□■□■□■□■□■□■□■□■□■■■■■■■\n\
            □□□□□□□□○           ■□□□■            □□□□□□□□\n\
            ○○○○○○■○○           ■■■■■            ○○○○○○○○\n",
            "      □                                      \n",
            "      ■                                      \n",
            "      □                                      \n",
            "      ■                                      \n",
            "      □                                      \n",
            "      ■                                      \n",
            "      □                                      \n",
            "      ■                                      \n",
            "      □                                      \n",
            "      ■                                      \n",
            "      □                                      \n",
            "    ■■■■■           ■■■■■           ■■■■■    \n",
            "    ■□□□■           ■□□□■           ■□□□■    \n",
            "    ■□■□■           ■□■□■           ■□■□■    \n",
            "    ■□□□■           ■□□□■           ■□□□■    \n",
            "    ■■■■■           ■■■■■           ■■■■■    \n",
            "      □                                      \n",
            "      ■                                      \n",
            "      □                                      \n",
            "      ■                                      \n",
            "      □                                      \n",
            "      ■                                      \n",
            "      □                                      \n",
            "      ■                                      \n",
            "      □                                      \n",
            "○○○○○○■                                      \n",
            "○○○○○○□                                      \n",
            "○○○○○○■             ■■■■■           ■■■■■    \n\
            □□□□□□□□■           ■□□□■           ■□□□■    \n\
            ■■■■■■■□○           ■□■□■           ■□■□■    \n\
            ■□□□□□■□○           ■□□□■           ■□□□■    \n\
            ■□■■■□■□○           ■■■■■           ■■■■■    \n\
            ■□■■■□■□○                                    \n\
            ■□■■■□■□○                                    \n\
            ■□□□□□■□○                                    \n\
            ■■■■■■■□○                                    "
        ].join("")
            .to_string()
    );
}

#[test]
fn testtest() {
    for x in (7..45).rev().step_by(2) {
        println!("{}", x);
    }
}