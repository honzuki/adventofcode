use std::{
    collections::{BTreeSet, HashMap, HashSet},
    str::FromStr,
};

const START: char = 'S';
const PLOT: char = '.';
const ROCK: char = '#';

#[derive(thiserror::Error, Debug)]
pub enum GardenErr {
    #[error("the garden can not be empty")]
    Empty,

    #[error("not all of the lines of the garden are of the same length")]
    NotEven,

    #[error("the start position is missing")]
    NoStart,

    #[error("unknown tile: {0}")]
    BadTile(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Plot,
    Rock,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position {
    col: usize,
    row: usize,
}

impl Position {
    fn new(col: usize, row: usize) -> Self {
        Self { col, row }
    }

    fn moves(self) -> [Option<Self>; 4] {
        let left = if self.col > 0 {
            Some(Self::new(self.col - 1, self.row))
        } else {
            None
        };
        let up = if self.row > 0 {
            Some(Self::new(self.col, self.row - 1))
        } else {
            None
        };
        let down = Some(Self::new(self.col, self.row + 1));
        let right = Some(Self::new(self.col + 1, self.row));

        [left, up, down, right]
    }

    fn moves_wrap(self, max_r: usize, max_c: usize) -> [Self; 4] {
        let left = if self.col > 0 {
            Self::new(self.col - 1, self.row)
        } else {
            Self::new(max_c - 1, self.row)
        };
        let up = if self.row > 0 {
            Self::new(self.col, self.row - 1)
        } else {
            Self::new(self.col, max_r - 1)
        };
        let down = if self.row < (max_r - 1) {
            Self::new(self.col, self.row + 1)
        } else {
            Self::new(self.col, 0)
        };
        let right = if self.col < (max_c - 1) {
            Self::new(self.col + 1, self.row)
        } else {
            Self::new(0, self.row)
        };

        [left, down, right, up]
    }
}

pub struct Map {
    data: Vec<Vec<Tile>>,
    start: Position,
}

impl Map {
    pub fn can_reach(&self, mut steps: usize) -> usize {
        let mut frontier = vec![self.start];

        while steps > 0 && !frontier.is_empty() {
            let mut new_frontier = HashSet::new();
            while let Some(pos) = frontier.pop() {
                new_frontier.extend(
                    pos.moves()
                        .into_iter()
                        .flatten()
                        .filter(|pos| pos.row < self.data.len() && pos.col < self.data[0].len())
                        .filter(|pos| matches!(self.data[pos.row][pos.col], Tile::Plot)),
                );
            }
            frontier = new_frontier.into_iter().collect();

            steps -= 1;
        }

        frontier.len()
    }

    pub fn can_reach_wrap(&self, mut steps: usize) -> usize {
        todo!();
    }
}

impl FromStr for Map {
    type Err = GardenErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = None;

        let data = s
            .trim()
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.trim()
                    .chars()
                    .enumerate()
                    .map(|(col, ch): (usize, char)| {
                        Ok(match ch {
                            PLOT => Tile::Plot,
                            ROCK => Tile::Rock,
                            START => {
                                start = Some(Position::new(col, row));
                                Tile::Plot
                            }
                            _ => return Err(GardenErr::BadTile(ch)),
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        if data.is_empty() {
            return Err(GardenErr::Empty);
        }
        let rlen = data[0].len();
        if data.iter().any(|row| row.len() != rlen) {
            return Err(GardenErr::NotEven);
        }

        let Some(start) = start else {
            return Err(GardenErr::NoStart);
        };

        Ok(Self { data, start })
    }
}

#[cfg(test)]
mod tests {
    use super::Map;

    #[test]
    fn can_reach() {
        let map: Map = r#"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
..........."#
            .parse()
            .unwrap();

        assert_eq!(map.can_reach(6), 16);
    }
}
