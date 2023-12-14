use std::{collections::VecDeque, str::FromStr};

const CUBE: char = '#';
const ROUNDED: char = 'O';
const EMPTY: char = '.';

#[derive(thiserror::Error, Debug)]
pub enum PrdErr {
    #[error("the platform contain an unknown space: {0}")]
    UnknownSpaceValue(char),

    #[error("the platform can not be empty")]
    Empty,

    #[error("not all of the lines in the platform are of the same length")]
    NotEven,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Space {
    Cube,
    Rounded,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Platform {
    data: Vec<Vec<Space>>,
}

impl Platform {
    pub fn tilt_cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    pub fn tilt_north(&mut self) {
        for cidx in 0..self.data[0].len() {
            let mut empty = VecDeque::new();
            for ridx in 0..self.data.len() {
                match self.data[ridx][cidx] {
                    Space::Empty => empty.push_back(ridx),
                    Space::Cube => empty.clear(),
                    Space::Rounded => {
                        if let Some(new_ridx) = empty.pop_front() {
                            self.data[new_ridx][cidx] = Space::Rounded;
                            self.data[ridx][cidx] = Space::Empty;
                            empty.push_back(ridx);
                        }
                    }
                }
            }
        }
    }

    pub fn tilt_west(&mut self) {
        for ridx in 0..self.data.len() {
            let mut empty = VecDeque::new();
            for cidx in 0..self.data[ridx].len() {
                match self.data[ridx][cidx] {
                    Space::Empty => empty.push_back(cidx),
                    Space::Cube => empty.clear(),
                    Space::Rounded => {
                        if let Some(new_cidx) = empty.pop_front() {
                            self.data[ridx][new_cidx] = Space::Rounded;
                            self.data[ridx][cidx] = Space::Empty;
                            empty.push_back(cidx);
                        }
                    }
                }
            }
        }
    }

    pub fn tilt_south(&mut self) {
        for cidx in 0..self.data[0].len() {
            let mut empty = VecDeque::new();
            for ridx in (0..self.data.len()).rev() {
                match self.data[ridx][cidx] {
                    Space::Empty => empty.push_back(ridx),
                    Space::Cube => empty.clear(),
                    Space::Rounded => {
                        if let Some(new_ridx) = empty.pop_front() {
                            self.data[new_ridx][cidx] = Space::Rounded;
                            self.data[ridx][cidx] = Space::Empty;
                            empty.push_back(ridx);
                        }
                    }
                }
            }
        }
    }

    pub fn tilt_east(&mut self) {
        for ridx in 0..self.data.len() {
            let mut empty = VecDeque::new();
            for cidx in (0..self.data[ridx].len()).rev() {
                match self.data[ridx][cidx] {
                    Space::Empty => empty.push_back(cidx),
                    Space::Cube => empty.clear(),
                    Space::Rounded => {
                        if let Some(new_cidx) = empty.pop_front() {
                            self.data[ridx][new_cidx] = Space::Rounded;
                            self.data[ridx][cidx] = Space::Empty;
                            empty.push_back(cidx);
                        }
                    }
                }
            }
        }
    }

    pub fn load_on_north_support_beam(&self) -> usize {
        self.data
            .iter()
            .rev()
            .enumerate()
            .map(|(ridx, line)| {
                line.iter()
                    .filter(|space| matches!(space, Space::Rounded))
                    .count()
                    * (ridx + 1)
            })
            .sum::<usize>()
    }
}

impl FromStr for Platform {
    type Err = PrdErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .trim()
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        CUBE => Ok(Space::Cube),
                        ROUNDED => Ok(Space::Rounded),
                        EMPTY => Ok(Space::Empty),
                        _ => Err(PrdErr::UnknownSpaceValue(ch)),
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        if data.is_empty() {
            return Err(PrdErr::Empty);
        }

        let rlen = data[0].len();
        if data.iter().any(|line| line.len() != rlen) {
            return Err(PrdErr::NotEven);
        }

        Ok(Self { data })
    }
}

#[cfg(test)]
mod tests {
    use super::{Platform, Space};

    #[test]
    fn parse_platform() {
        let input = r#"O....
        O.OO#"#;

        let expected_output = Platform {
            data: vec![
                vec![
                    Space::Rounded,
                    Space::Empty,
                    Space::Empty,
                    Space::Empty,
                    Space::Empty,
                ],
                vec![
                    Space::Rounded,
                    Space::Empty,
                    Space::Rounded,
                    Space::Rounded,
                    Space::Cube,
                ],
            ],
        };

        let output: Platform = input.parse().unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn tilt_north_simple() {
        let input = r#"O....
O.OO#"#;
        let expected_output = Platform {
            data: vec![
                vec![
                    Space::Rounded,
                    Space::Empty,
                    Space::Rounded,
                    Space::Rounded,
                    Space::Empty,
                ],
                vec![
                    Space::Rounded,
                    Space::Empty,
                    Space::Empty,
                    Space::Empty,
                    Space::Cube,
                ],
            ],
        };

        let mut output: Platform = input.parse().unwrap();
        output.tilt_north();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn calculate_load() {
        let input = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#;
        let expected_output = 136;

        let mut output: Platform = input.parse().unwrap();
        output.tilt_north();
        let output = output.load_on_north_support_beam();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn tilt_west() {
        let input: Platform = ".O.O#.O#O..O".parse().unwrap();
        let expected_output = Platform {
            data: vec![vec![
                Space::Rounded,
                Space::Rounded,
                Space::Empty,
                Space::Empty,
                Space::Cube,
                Space::Rounded,
                Space::Empty,
                Space::Cube,
                Space::Rounded,
                Space::Rounded,
                Space::Empty,
                Space::Empty,
            ]],
        };

        let mut output = input;
        output.tilt_west();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn tilt_south() {
        let input: Platform = r#".
O
.
O
.
.
#
O
.
#
.
O
."#
        .parse()
        .unwrap();
        let expected_output = Platform {
            data: vec![
                vec![Space::Empty],
                vec![Space::Empty],
                vec![Space::Empty],
                vec![Space::Empty],
                vec![Space::Rounded],
                vec![Space::Rounded],
                vec![Space::Cube],
                vec![Space::Empty],
                vec![Space::Rounded],
                vec![Space::Cube],
                vec![Space::Empty],
                vec![Space::Empty],
                vec![Space::Rounded],
            ],
        };

        let mut output = input;
        output.tilt_south();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn tilt_east() {
        let input: Platform = "O.O.#.O#O..O".parse().unwrap();
        let expected_output = Platform {
            data: vec![vec![
                Space::Empty,
                Space::Empty,
                Space::Rounded,
                Space::Rounded,
                Space::Cube,
                Space::Empty,
                Space::Rounded,
                Space::Cube,
                Space::Empty,
                Space::Empty,
                Space::Rounded,
                Space::Rounded,
            ]],
        };

        let mut output = input;
        output.tilt_east();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn complete_cycle() {
        let input: Platform = r#"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."#
            .parse()
            .unwrap();
        let expected_output: Platform = r#".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
"#
        .parse()
        .unwrap();

        let mut output = input;
        output.tilt_cycle();
        assert_eq!(output, expected_output);

        let expected_output: Platform = r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
        "#
        .parse()
        .unwrap();

        output.tilt_cycle();
        output.tilt_cycle();
        assert_eq!(output, expected_output);
    }
}
