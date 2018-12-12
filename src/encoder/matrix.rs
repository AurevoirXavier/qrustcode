// state: u8
// 0 -> 0 normal module
// 1 -> 1 normal module
// 2 -> 0 function module
// 3 -> 1 function module
// 4 -> 0 reserved module
// 5 -> 0 unused module
#[derive(Debug)]
pub struct Matrix(Vec<Vec<u8>>);

fn normalize_module(module: u8) -> u8 {
    match module {
        1 | 3 => 1,
        _ => 0,
    }
}

impl Matrix {
    fn add_finder_patterns(&mut self) -> &mut Matrix {
        let Matrix(matrix) = self;
        let fix = matrix.len() - 7;

        for (i, j) in [(0, 0), (fix, 0), (0, fix)].iter() {
            for y in 0..7 {
                for x in 0..7 {
                    matrix[x + i][y + j] = match x {
                        1 | 5 => match y {
                            0 => 3,
                            _ => 2,
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

    fn eval_condition_1(matrix: &Vec<Vec<u8>>) -> u32 {
        let mut penalty = 0;

        let edge = matrix.len();
        for i in 0..edge {
            let mut count_x = 1u8;
            let mut prev_module_x = &matrix[i][0];
            let mut count_y = 1u8;
            let mut prev_module_y = &matrix[0][i];

            for j in 1..edge {
                // horizontal
                let module = &matrix[i][j];
                if module == prev_module_x {
                    count_x += 1;
                    if count_x == 5 { penalty += 3; } else if count_x > 5 { penalty += 1; }
                } else {
                    prev_module_x = module;
                    count_x = 1;
                }

                // vertical
                let module = &matrix[j][i];
                if module == prev_module_y {
                    count_y += 1;
                    if count_y == 5 { penalty += 3; } else if count_y > 5 { penalty += 1; }
                } else {
                    prev_module_y = module;
                    count_y = 1;
                }
            }
        }

        penalty
    }

    fn eval_condition_2(matrix: &Vec<Vec<u8>>) -> u32 {
        let mut penalty = 0;

        let edge = matrix.len() - 1;
        for y in 0..edge {
            for x in 0..edge {
                let top_left = &matrix[y][x];

                let top_right = &matrix[y][x + 1];
                if top_left != top_right { continue; }

                let bottom_left = &matrix[y + 1][x];
                if top_left != bottom_left { continue; }

                let bottom_right = &matrix[y + 1][x + 1];
                if top_left != bottom_right { continue; }

                penalty += 3;
            }
        }

        penalty
    }

    fn eval_condition_3(matrix: &Vec<Vec<u8>>) -> u32 {
        let mut penalty = 0;

        let edge = matrix.len();
        for i in 0..edge {
            for j in 0..edge - 10 {
                // horizontal
                let modules = &matrix[i][j..j + 11];
                if modules == [1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0] || modules == [0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1] { penalty += 40; }

                // vertical
                let modules: Vec<u8> = matrix[j..j + 11].iter().map(|y| y[i]).collect();
                if modules == [1, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0] || modules == [0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1] { penalty += 40; }
            }
        }

        penalty
    }

    fn eval_condition_4(matrix: &Vec<Vec<u8>>) -> u32 {
        let percent_of_dark_modules = {
            let mut dark_modules = 0;
            for row in matrix {
                for module in row {
                    if module == &0 { dark_modules += 1; }
                }
            }

            (dark_modules * 100) / (matrix.len() as i32).pow(2) * 5 / 5
        };

        ((50 - percent_of_dark_modules).abs() as u32 / 5).min((55 - percent_of_dark_modules).abs() as u32 / 5) * 10
    }

    fn data_mask(&mut self) -> Matrix {
        use std::{
            sync::Arc,
            thread,
        };

        fn mask_1(x: u8, y: u8) -> bool { (x as u16 + y as u16) % 2 == 0 }
        fn mask_2(x: u8, _: u8) -> bool { x % 2 == 0 }
        fn mask_3(_: u8, y: u8) -> bool { y % 3 == 0 }
        fn mask_4(x: u8, y: u8) -> bool { (x as u16 + y as u16) % 3 == 0 }
        fn mask_5(x: u8, y: u8) -> bool { ((x as f32 / 2.).floor() + (y as f32 / 2.).floor()) as u8 % 2 == 0 }
        fn mask_6(x: u8, y: u8) -> bool { ((x as u16 * y as u16) % 2) + ((x as u16 * y as u16) % 3) == 0 }
        fn mask_7(x: u8, y: u8) -> bool { (((x as u16 * y as u16) % 2) + ((x as u16 * y as u16) % 3)) % 2 == 0 }
        fn mask_8(x: u8, y: u8) -> bool { (((x as u16 + y as u16) % 2) + ((x as u16 * y as u16) % 3)) % 2 == 0 }

        let Matrix(matrix) = self;

        let mut handlers = vec![];
        for mask in [mask_1, mask_2, mask_3, mask_4, mask_5, mask_6, mask_7, mask_8].iter() {
            let mut matrix = matrix.clone();
            let handler = thread::spawn(move || {
                let edge = matrix.len();
                for y in 0..edge {
                    for x in 0..edge {
                        match matrix[y][x] {
                            0 if mask(x as u8, y as u8) => matrix[y][x] = 1,
                            1 if mask(x as u8, y as u8) => matrix[y][x] = 0,
                            _ => continue,
                        }
                    }
                }

                let matrix = Arc::new(matrix);
                let mut handlers = vec![];
                for eval_condition in [
                    Matrix::eval_condition_1,
                    Matrix::eval_condition_2,
                    Matrix::eval_condition_3,
                    Matrix::eval_condition_4,
                ].iter() {
                    let matrix = Arc::clone(&matrix);
                    let handler = thread::spawn(move || eval_condition(&matrix));

                    handlers.push(handler)
                }

                (
                    handlers.into_iter()
                        .map(|handler| handler.join().unwrap())
                        .sum::<u32>(),
                    Matrix(Arc::try_unwrap(matrix).unwrap())
                )
            });

            handlers.push(handler);
        }

        handlers.into_iter()
            .map(|handler| handler.join().unwrap())
            .min_by_key(|&(penalty, _)| penalty)
            .unwrap()
            .1
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
            .add_dark_module_and_reserved_areas(version)
            .place_data(data)
            .data_mask();

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
    );
}

#[test]
fn test_eval_condition() {
    let matrix = vec![
        vec![1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 1],
        vec![1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 1, 1, 0, 1],
        vec![1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1],
        vec![1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 1],
        vec![0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1],
        vec![0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0],
        vec![1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0],
        vec![1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 1, 0, 1, 1, 0, 1, 0, 0, 0],
        vec![1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1],
        vec![1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0],
        vec![1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 0, 1],
        vec![1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 1],
        vec![1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 1]
    ];

    assert_eq!(Matrix::eval_condition_1(&matrix), 180);
    assert_eq!(Matrix::eval_condition_2(&matrix), 90);
    assert_eq!(Matrix::eval_condition_3(&matrix), 80);
    assert_eq!(Matrix::eval_condition_4(&matrix), 0);
}
