// state: u8
// 0 -> 0
// 1 -> 1
// 2 -> function module 0
// 3 -> function module 1
// 4 -> reserved
// 5 -> unused
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
                            1...5 => 2,
                            _ => 3
                        }
                        2...4 => match y {
                            1 | 5 => 2,
                            _ => 3
                        }
                        _ => 3
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
            matrix[7][i] = 2; // horizontal
            matrix[i][7] = 2; // vertical
            // top right
            matrix[i][fix] = 2; // vertical
            // bottom left
            matrix[fix][i] = 2; // horizontal
        }

        for i in fix..len {
            // top right
            matrix[7][i] = 2; // horizontal
            // bottom left
            matrix[i][7] = 2; // vertical
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
                if matrix[x as usize][y as usize] == 3 { continue; }

                // get top left module coordinate
                let (i, j) = (x as usize - 2, y as usize - 2);

                for y in 0..5 {
                    for x in 0..5 {
                        matrix[x + i][y + j] = match x {
                            1...3 => match y {
                                1...3 => 2,
                                _ => 3
                            }
                            _ => 3
                        }
                    }
                }

                // center module
                matrix[x as usize][y as usize] = 3;
            }
        }

        self
    }

    fn add_timing_patterns(&mut self) -> &mut Matrix {
        let Matrix(matrix) = self;
        let fix = matrix.len() - 8;
        let mut timing_pattern = [3, 3, 2, 2].iter().cycle();

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
            matrix[8][i] = 4;
            // vertical
            matrix[i][8] = 4;
        }

        for i in fix..len {
            // horizontal
            matrix[8][i] = 4;
            // vertical
            matrix[i][8] = 4;
        }

        // avoid timing pattern
        matrix[6][8] = 3;
        matrix[8][6] = 3;
        // avoid dark module
        matrix[fix][8] = 3;

        if version > 6 {
            for i in fix - 3..fix {
                for j in 0..6 {
                    // top-right
                    matrix[i][j] = 4;
                    // bottom-left
                    matrix[j][i] = 4;
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
        let mut x = len - 1;

        'outer: loop {
            if upward {
                for y in (0..len).rev() {
                    if matrix[y][x] == 5 { if let Some(&bit) = data.next() { matrix[y][x] = bit; } else { break 'outer; } }
                    if matrix[y][x - 1] == 5 { if let Some(&bit) = data.next() { matrix[y][x - 1] = bit; } else { break 'outer; } }
                }
            } else {
                for y in 0..len {
                    if matrix[y][x] == 5 { if let Some(&bit) = data.next() { matrix[y][x] = bit; } else { break 'outer; } }
                    if matrix[y][x - 1] == 5 { if let Some(&bit) = data.next() { matrix[y][x - 1] = bit; } else { break 'outer; } }
                }
            };

            upward = !upward;

            match x {
                1 => break,
                8 => x -= 3, // avoid timing pattern
                _ => x -= 2
            }
        }

        self
    }

    fn data_mask(&mut self) -> &mut Matrix {
        unimplemented!()
    }

    pub fn new(data: &Vec<u8>, version: usize) -> Matrix {
        let mut matrix = Matrix({
            let size = version * 4 + 17;
            let mut matrix = vec![];
            let mut row = vec![];

            row.resize(size, 5);
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

        row.resize(21, 5);
        matrix.resize(21, row);

        matrix
    });

    matrix.add_finder_patterns();

    assert_eq!(
        matrix.0.iter()
            .map(|row| row.iter()
                .map(|state| match state {
                    2 => '□',
                    3 => '■',
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

        row.resize(21, 5);
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
                    2 => '□',
                    3 => '■',
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

        row.resize(25, 5);
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
                    2 => '□',
                    3 => '■',
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

        row.resize(25, 5);
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
                    2 => '□',
                    3 => '■',
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

        row.resize(25, 5);
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
                    2 => '□',
                    3 => '■',
                    4 => '○',
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

        row.resize(45, 5);
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
                    2 => '□',
                    3 => '■',
                    4 => '○',
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
fn test_place_data() {
    let mut matrix = Matrix({
        let mut matrix = vec![];
        let mut row = vec![];

        row.resize(25, 5);
        matrix.resize(25, row);

        matrix
    });

    let mut data = vec![];
    for _ in 0..18 { data.extend_from_slice(&[1, 0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1]); }
    for _ in 0..17 { data.push(0); }

    matrix
        .add_finder_patterns()
        .add_separators()
        .add_alignment_patterns(2)
        .add_timing_patterns()
        .add_dark_module_and_reserved_areas(2)
        .place_data(&data);

    assert_eq!(
        matrix.0.iter()
            .map(|row| row.iter()
                .map(|state| match state {
                    0 => '0',
                    1 => '1',
                    2 => '□',
                    3 => '■',
                    4 => '○',
                    _ => ' '
                }).collect::<String>()
            ).collect::<Vec<String>>()
            .join("\n"),
        [
            "\
            ■■■■■■■□○01111110□■■■■■■■\n\
            ■□□□□□■□○01011010□■□□□□□■\n\
            ■□■■■□■□○00010010□■□■■■□■\n\
            ■□■■■□■□○10010010□■□■■■□■\n\
            ■□■■■□■□○11011111□■□■■■□■\n\
            ■□□□□□■□○10110110□■□□□□□■\n\
            ■■■■■■■□■□■□■□■□■□■■■■■■■\n\
            □□□□□□□□○10000100□□□□□□□□\n\
            ○○○○○○■○○10000101○○○○○○○○\n",
            "000111□001010010110110101\n",
            "001001■001111111110100101\n",
            "001001□101010010110001111\n",
            "001001■110010010110010100\n",
            "001001□100010000111010100\n",
            "001111■101110100110110010\n",
            "001001□100111111100011011\n",
            "000001■100110100■■■■■1110\n\
            □□□□□□□□■0100100■□□□■1010\n\
            ■■■■■■■□○0101100■□■□■1010\n\
            ■□□□□□■□○1101100■□□□■1010\n\
            ■□■■■□■□○0111111■■■■■1011\n\
            ■□■■■□■□○0101100100011110\n\
            ■□■■■□■□○0001001111011000\n\
            ■□□□□□■□○1001000101010001\n\
            ■■■■■■■□○1101110101110001"
        ].join("")
            .to_string()
    )
}