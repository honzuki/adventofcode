use std::{collections::HashSet, str::FromStr};

const EMPTY: char = '.';
const LEFT_TILTED_MIRROR: char = '\\';
const RIGHT_TILTED_MIRROR: char = '/';
const VERTICAL_SPLITTER: char = '|';
const HORIZONTAL_SPLITTER: char = '-';

#[derive(thiserror::Error, Debug)]
pub enum GridErr {
    #[error("the grid can not be empty")]
    Empty,

    #[error("not all of the lines in the grid are of the same length")]
    UnEven,

    #[error("unknown space value: {0}")]
    UnknownSpace(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Space {
    Empty,
    LeftTiltedMirror,
    RightTiltedMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl Space {
    fn calculate_next_dir(&self, dir: Dir) -> [Option<Dir>; 2] {
        match (self, dir) {
            (Self::LeftTiltedMirror, _) => [
                Some(match dir {
                    Dir::Down => Dir::Right,
                    Dir::Left => Dir::Up,
                    Dir::Right => Dir::Down,
                    Dir::Up => Dir::Left,
                }),
                None,
            ],
            (Self::RightTiltedMirror, _) => [
                Some(match dir {
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Down,
                    Dir::Right => Dir::Up,
                    Dir::Up => Dir::Right,
                }),
                None,
            ],
            (Self::VerticalSplitter, Dir::Right | Dir::Left) => [Some(Dir::Up), Some(Dir::Down)],
            (Self::HorizontalSplitter, Dir::Down | Dir::Up) => [Some(Dir::Left), Some(Dir::Right)],
            _ => [Some(dir), None],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir {
    Left,
    Down,
    Right,
    Up,
}

impl Dir {
    fn apply(self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Self::Left => {
                if pos.1 == 0 {
                    None
                } else {
                    Some((pos.0, pos.1 - 1))
                }
            }
            Self::Up => {
                if pos.0 == 0 {
                    None
                } else {
                    Some((pos.0 - 1, pos.1))
                }
            }
            Self::Down => Some((pos.0 + 1, pos.1)),
            Self::Right => Some((pos.0, pos.1 + 1)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    data: Vec<Vec<Space>>,
}

impl Grid {
    pub fn calculate_energized(&self, start: (usize, usize), dir: Dir) -> usize {
        let mut frontier = vec![(dir, start)];
        let mut visited_spaces: HashSet<(usize, usize)> = Default::default();
        let mut visited: HashSet<(Dir, (usize, usize))> = Default::default();

        while let Some((dir, (row, col))) = frontier.pop() {
            if row >= self.data.len() || col >= self.data[0].len() {
                continue;
            }
            visited_spaces.insert((row, col));

            if !visited.insert((dir, (row, col))) {
                continue;
            }

            let space = self.data[row][col];
            frontier.extend(
                space
                    .calculate_next_dir(dir)
                    .into_iter()
                    .flatten()
                    .filter_map(|dir| dir.apply((row, col)).map(|pos| (dir, pos))),
            )
        }

        visited_spaces.len()
    }

    pub fn len(&self) -> (usize, usize) {
        (self.data.len(), self.data[0].len())
    }
}

impl FromStr for Grid {
    type Err = GridErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .trim()
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|ch| {
                        Ok(match ch {
                            EMPTY => Space::Empty,
                            LEFT_TILTED_MIRROR => Space::LeftTiltedMirror,
                            RIGHT_TILTED_MIRROR => Space::RightTiltedMirror,
                            VERTICAL_SPLITTER => Space::VerticalSplitter,
                            HORIZONTAL_SPLITTER => Space::HorizontalSplitter,
                            _ => return Err(GridErr::UnknownSpace(ch)),
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        if data.is_empty() {
            return Err(GridErr::Empty);
        }

        let rlen = data[0].len();
        if data.iter().any(|line| line.len() != rlen) {
            return Err(GridErr::UnEven);
        }

        Ok(Self { data })
    }
}

#[cfg(test)]
mod tests {
    use super::{Dir, Grid, Space};

    #[test]
    fn parse_data() {
        let input = r#".|...\"#;
        let expected_output = Grid {
            data: vec![vec![
                Space::Empty,
                Space::VerticalSplitter,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::LeftTiltedMirror,
            ]],
        };

        let output: Grid = input.parse().unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn calculate_next_dir() {
        let space = Space::LeftTiltedMirror;
        let dir = Dir::Right;
        assert_eq!(
            space
                .calculate_next_dir(dir)
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
            &[Dir::Down]
        );
    }

    #[test]
    fn calc_energized() {
        let input: Grid = r#".|...\...."#.parse().unwrap();
        assert_eq!(input.calculate_energized((0, 0), Dir::Right), 2);
    }
}
