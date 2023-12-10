use std::str::FromStr;

#[derive(thiserror::Error, Debug)]
pub enum SketchErr {
    #[error("unknown tile: {0}")]
    UnknownTile(char),

    #[error("the sketch is missing the start position")]
    NoStart,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl Pipe {
    /// Get the 2 ends of a pipe from its position
    pub fn ends(&self, position: Point) -> [Option<Point>; 2] {
        match self {
            Self::Vertical => [position.north(), position.south()],
            Self::Horizontal => [position.west(), position.east()],
            Self::NorthEast => [position.north(), position.east()],
            Self::NorthWest => [position.north(), position.west()],
            Self::SouthEast => [position.south(), position.east()],
            Self::SouthWest => [position.south(), position.west()],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Pipe(Pipe),
    Ground,
    Start,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn all_dirs(&self) -> [Option<Point>; 4] {
        [self.north(), self.east(), self.south(), self.west()]
    }

    pub fn west(&self) -> Option<Point> {
        if self.x == 0 {
            None
        } else {
            Some(Self {
                x: self.x - 1,
                y: self.y,
            })
        }
    }

    pub fn east(&self) -> Option<Point> {
        if self.x == usize::MAX {
            None
        } else {
            Some(Self {
                x: self.x + 1,
                y: self.y,
            })
        }
    }

    pub fn north(&self) -> Option<Point> {
        if self.y == 0 {
            None
        } else {
            Some(Self {
                x: self.x,
                y: self.y - 1,
            })
        }
    }

    pub fn south(&self) -> Option<Point> {
        if self.y == usize::MAX {
            None
        } else {
            Some(Self {
                x: self.x,
                y: self.y + 1,
            })
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sketch {
    pub tiles: Vec<Vec<Tile>>,
}

impl Sketch {
    /// Get the start position
    pub fn start(&self) -> Option<Point> {
        self.tiles.iter().enumerate().find_map(|(rdx, row)| {
            row.iter().enumerate().find_map(|(cdx, col)| {
                if matches!(col, Tile::Start) {
                    Some(Point { x: cdx, y: rdx })
                } else {
                    None
                }
            })
        })
    }

    /// Get the tile at the given position, if exists
    pub fn tile(&self, position: Point) -> Option<Tile> {
        self.tiles
            .get(position.y)
            .and_then(|row| row.get(position.x))
            .copied()
    }
}

impl FromStr for Sketch {
    type Err = SketchErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| {
                        Ok(match ch {
                            '|' => Tile::Pipe(Pipe::Vertical),
                            '-' => Tile::Pipe(Pipe::Horizontal),
                            'L' => Tile::Pipe(Pipe::NorthEast),
                            'J' => Tile::Pipe(Pipe::NorthWest),
                            '7' => Tile::Pipe(Pipe::SouthWest),
                            'F' => Tile::Pipe(Pipe::SouthEast),
                            '.' => Tile::Ground,
                            'S' => Tile::Start,
                            _ => return Err(SketchErr::UnknownTile(ch)),
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Sketch { tiles })
    }
}

#[cfg(test)]
mod tests {
    use super::{Pipe, Point, Sketch, Tile};

    #[test]
    fn parse_sketch() {
        let input = r#".....
.F-7.
.|.|.
.L-J.
....."#;
        let expected_output = Sketch {
            tiles: vec![
                vec![
                    Tile::Ground,
                    Tile::Ground,
                    Tile::Ground,
                    Tile::Ground,
                    Tile::Ground,
                ],
                vec![
                    Tile::Ground,
                    Tile::Pipe(Pipe::SouthEast),
                    Tile::Pipe(Pipe::Horizontal),
                    Tile::Pipe(Pipe::SouthWest),
                    Tile::Ground,
                ],
                vec![
                    Tile::Ground,
                    Tile::Pipe(Pipe::Vertical),
                    Tile::Ground,
                    Tile::Pipe(Pipe::Vertical),
                    Tile::Ground,
                ],
                vec![
                    Tile::Ground,
                    Tile::Pipe(Pipe::NorthEast),
                    Tile::Pipe(Pipe::Horizontal),
                    Tile::Pipe(Pipe::NorthWest),
                    Tile::Ground,
                ],
                vec![
                    Tile::Ground,
                    Tile::Ground,
                    Tile::Ground,
                    Tile::Ground,
                    Tile::Ground,
                ],
            ],
        };

        let output: Sketch = input.parse().unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn find_start_position() {
        let input: Sketch = r#".....
.S-7.
.|.|.
.L-J.
....."#
            .parse()
            .unwrap();
        let expected_output = Point { x: 1, y: 1 };

        assert_eq!(input.start(), Some(expected_output));
    }

    #[test]
    fn pipe_transition() {
        let pipe = Pipe::SouthWest;
        let ends = pipe.ends(Point {
            x: 0,
            y: usize::MAX,
        });
        assert_eq!(ends, [None, None]);

        let pipe = Pipe::NorthEast;
        let ends = pipe.ends(Point { x: 5, y: 8 });
        let expected_ends = [Some(Point { x: 5, y: 7 }), Some(Point { x: 6, y: 8 })];
        assert_eq!(ends, expected_ends);
    }
}
