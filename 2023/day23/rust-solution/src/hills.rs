use std::{collections::HashSet, str::FromStr};

const PATH: char = '.';
const FOREST: char = '#';
const SLOPE_UP: char = '^';
const SLOPE_DOWN: char = 'v';
const SLOPE_RIGHT: char = '>';
const SLOPE_LEFT: char = '<';

#[derive(thiserror::Error, Debug)]
pub enum HillsErr {
    #[error("the trail can not be empty")]
    Empty,

    #[error("not all the rows in the trail are of the same length")]
    NotEven,

    #[error("unknown tile: {0}")]
    UnknownTile(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Slope {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Path,
    Forest,
    Slope(Slope),
}
impl Tile {
    fn from_char(ch: char) -> Result<Self, HillsErr> {
        Ok(match ch {
            PATH => Self::Path,
            FOREST => Self::Forest,
            SLOPE_UP => Self::Slope(Slope::Up),
            SLOPE_DOWN => Self::Slope(Slope::Down),
            SLOPE_LEFT => Self::Slope(Slope::Left),
            SLOPE_RIGHT => Self::Slope(Slope::Right),
            _ => return Err(HillsErr::UnknownTile(ch)),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn left(self) -> Option<Self> {
        if self.x > 0 {
            Some(Position::new(self.x - 1, self.y))
        } else {
            None
        }
    }

    fn up(self) -> Option<Self> {
        if self.y > 0 {
            Some(Position::new(self.x, self.y - 1))
        } else {
            None
        }
    }

    fn right(self) -> Option<Self> {
        if self.x < usize::MAX {
            Some(Position::new(self.x + 1, self.y))
        } else {
            None
        }
    }

    fn down(self) -> Option<Self> {
        if self.y < usize::MAX {
            Some(Position::new(self.x, self.y + 1))
        } else {
            None
        }
    }

    fn moves(self) -> [Option<Self>; 4] {
        [self.up(), self.right(), self.down(), self.left()]
    }
}

pub struct Trail {
    data: Vec<Vec<Tile>>,
}

impl Trail {
    pub fn find_longest_path(&self) -> Option<usize> {
        todo!()
    }

    pub fn find_longest_slippery_path(&self) -> Option<usize> {
        let start = Position::new(1, 0);
        let mut visited = HashSet::from([start]);
        self.find_longest_slippery_path_dfs(&mut visited, 0, start)
    }

    fn find_longest_slippery_path_dfs(
        &self,
        visited: &mut HashSet<Position>,
        steps: usize,
        current: Position,
    ) -> Option<usize> {
        if current == Position::new(self.data[0].len() - 2, self.data.len() - 1) {
            return Some(steps);
        }

        let next = match self.data[current.y][current.x] {
            Tile::Path => current.moves(),
            Tile::Slope(Slope::Down) => [current.down(), None, None, None],
            Tile::Slope(Slope::Up) => [current.up(), None, None, None],
            Tile::Slope(Slope::Left) => [current.left(), None, None, None],
            Tile::Slope(Slope::Right) => [current.right(), None, None, None],
            Tile::Forest => unreachable!(),
        };

        let mut best: Option<usize> = None;
        for next in next.into_iter().flatten().filter(|position| {
            position.y < self.data.len()
                && position.x < self.data[0].len()
                && !matches!(self.data[position.y][position.x], Tile::Forest)
        }) {
            if !visited.insert(next) {
                continue;
            }

            if let Some(max) = self.find_longest_slippery_path_dfs(visited, steps + 1, next) {
                best = Some(match best {
                    Some(best) => best.max(max),
                    None => max,
                })
            }

            visited.remove(&next);
        }

        best
    }
}

impl FromStr for Trail {
    type Err = HillsErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .trim()
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(Tile::from_char)
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        if data.is_empty() {
            return Err(HillsErr::Empty);
        }

        let rlen = data[0].len();
        if data.iter().any(|row| row.len() != rlen) {
            return Err(HillsErr::NotEven);
        }

        Ok(Self { data })
    }
}
