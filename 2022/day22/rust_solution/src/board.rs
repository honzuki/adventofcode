use std::{collections::HashMap, str::FromStr};

use crate::helper::split_many_whitespace_at_start;
use crate::moves::Movement;

type Position = (usize, usize);

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Tile {
    Open,
    Wall,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn rotate_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    fn rotate_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board {
    data: HashMap<usize, HashMap<usize, Tile>>,
    edges: HashMap<Position, HashMap<Position, Position>>,
    position: Option<Position>,
    direction: Direction,
}

impl Board {
    fn new() -> Board {
        Board {
            data: HashMap::new(),
            edges: HashMap::new(),
            position: None,
            direction: Direction::Right,
        }
    }

    fn set_tile(&mut self, tile: Tile, x: usize, y: usize) {
        self.data
            .entry(x)
            .and_modify(|row| {
                row.insert(y, tile.clone());
            })
            .or_insert(HashMap::from_iter(vec![(y, tile)]));
    }

    fn set_edge(&mut self, from: Position, to: Position, dest: Position) {
        self.edges
            .entry(from)
            .and_modify(|edges| {
                let test = edges.insert(to, dest);
                assert_eq!(test.is_none(), true);
            })
            .or_insert(HashMap::from_iter(vec![(to, dest)]));
    }

    pub fn get_current_password(&self) -> usize {
        (1000 * self.position.unwrap().1)
            + (4 * self.position.unwrap().0)
            + (self.direction.clone() as usize)
    }

    pub fn execute_movement(&mut self, movement: &Movement) {
        match movement {
            Movement::Step(count) => self.step(*count),
            Movement::RotateLeft => self.direction = self.direction.rotate_left(),
            Movement::RotateRight => self.direction = self.direction.rotate_right(),
        }
    }

    fn step(&mut self, count: usize) {
        for _ in 0..count {
            let (sx, sy) = self.position.unwrap();

            let (x, y) = match self.direction {
                Direction::Up => (sx, sy - 1),
                Direction::Down => (sx, sy + 1),
                Direction::Left => (sx - 1, sy),
                Direction::Right => (sx + 1, sy),
            };

            let (x, y) = if !self.data.contains_key(&x) || !self.data[&x].contains_key(&y) {
                // an edge
                self.edges[&(sx, sy)][&(x, y)]
            } else {
                (x, y)
            };

            let (x, y) = match &self.data[&x][&y] {
                Tile::Open => (x, y),
                Tile::Wall => {
                    return;
                }
            };

            self.position = Some((x, y));
        }
    }

    pub fn from_cubic_str(s: &str) -> Result<Self, &'static str> {
        let mut board = Board::new();

        let cube_len = s.lines().take(1).collect::<Vec<&str>>()[0].trim().len() / 2;
        board.position = Some((cube_len + 1, 1));

        for x in 1..=cube_len {
            // assuming the following shape:
            //       [0]  [1]  [2]
            //[0]       | 0 | 1|
            //[1]   __ | 2 |
            //[2] | 4 | 3 |
            //[3] | 5 |

            // 0::LEFT(start - 1, x) = 4::LEFT(start, rev(x))
            board.set_edge(
                (cube_len + 1, x),
                (cube_len, x),
                (1, (cube_len * 2) + (cube_len - x + 1)),
            );
            // 0::UP(x, start - 1) = 5::LEFT(start, x)
            board.set_edge(
                (cube_len + x, 1),
                (cube_len + x, 0),
                (1, (cube_len * 3) + x),
            );

            // 1::UP(x, start - 1) = 5::DOWN(x, end)
            board.set_edge(
                ((cube_len * 2) + x, 1),
                ((cube_len * 2) + x, 0),
                (x, cube_len * 4),
            );
            // 1::RIGHT(end + 1, x) = 3::RIGHT(end, rev(x))
            board.set_edge(
                (cube_len * 3, x),
                ((cube_len * 3) + 1, x),
                (cube_len * 2, (cube_len * 2) + (cube_len - x + 1)),
            );

            // 1::DOWN(x, end + 1) = 2::RIGHT(end, x)
            board.set_edge(
                ((cube_len * 2) + x, cube_len),
                ((cube_len * 2) + x, cube_len + 1),
                (cube_len * 2, cube_len + x),
            );

            // 2::LEFT(start - 1, x) = 4::UP(x, start)
            board.set_edge(
                (cube_len + 1, cube_len + x),
                (cube_len, cube_len + x),
                (x, cube_len * 2 + 1),
            );
            // 2::RIGHT(end + 1, x) = 1::DOWN(x, end)
            board.set_edge(
                (cube_len * 2, cube_len + x),
                ((cube_len * 2) + 1, cube_len + x),
                ((cube_len * 2) + x, cube_len),
            );

            // 3::RIGHT(end + 1, y) = 1::RIGHT(end, rev(y))
            board.set_edge(
                (cube_len * 2, (cube_len * 2) + x),
                ((cube_len * 2) + 1, (cube_len * 2) + x),
                (cube_len * 3, cube_len - x + 1),
            );
            // 3::DOWN(x, end + 1) = 5::RIGHT(end, x)
            board.set_edge(
                (cube_len + x, cube_len * 3),
                (cube_len + x, (cube_len * 3) + 1),
                (cube_len, (cube_len * 3) + x),
            );

            // 4::UP(x, start - 1) = 2::LEFT(start, x)
            board.set_edge(
                (x, (cube_len * 2) + 1),
                (x, cube_len * 2),
                (cube_len + 1, cube_len + x),
            );

            // 4::LEFT(start - 1, x) = 0::LEFT(start, rev(x))
            board.set_edge(
                (1, (cube_len * 2) + x),
                (0, (cube_len * 2) + x),
                (cube_len + 1, cube_len - x + 1),
            );

            // 5::LEFT(start - 1, x) = 0::UP(x, start)
            board.set_edge(
                (1, (cube_len * 3) + x),
                (0, (cube_len * 3) + x),
                (cube_len + x, 1),
            );
            // 5::DOWN(x, end + 1) = 1::UP(x, start)
            board.set_edge(
                (x, cube_len * 4),
                (x, (cube_len * 4) + 1),
                (cube_len + x, 1),
            );
            //5::RIGHT(end + 1, x) = 3::DOWN(x, end)
            board.set_edge(
                (cube_len, (cube_len * 3) + x),
                (cube_len + 1, (cube_len * 3) + x),
                (cube_len + x, cube_len * 3),
            );
        }

        for (y, line) in s.lines().enumerate() {
            let y = y + 1;

            let (line, x) = split_many_whitespace_at_start(line);
            for (x_offs, ch) in line.chars().enumerate() {
                let x = x + 1 + x_offs;

                let tile = match ch {
                    '.' => Tile::Open,
                    '#' => Tile::Wall,
                    _ => return Err("unknown tile"),
                };

                board.set_tile(tile, x, y);
            }
        }

        Ok(board)
    }
}
impl FromStr for Board {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Board::new();
        let mut min_max_y = HashMap::new();

        for (y, line) in s.lines().enumerate() {
            let (line, x) = split_many_whitespace_at_start(line);
            let y = y + 1;

            board.set_edge((x + 1, y), (x, y), (x + line.len(), y));
            board.set_edge((x + line.len(), y), (x + line.len() + 1, y), (x + 1, y));
            for (x_offs, ch) in line.chars().enumerate() {
                let x = x + 1 + x_offs;

                let tile = match ch {
                    '.' => Tile::Open,
                    '#' => Tile::Wall,
                    _ => return Err("unknown tile"),
                };

                board.set_tile(tile, x, y);

                min_max_y
                    .entry(x)
                    .and_modify(|(min_y, max_y)| {
                        *min_y = (y - 1).min(*min_y);
                        *max_y = (y + 1).max(*max_y);
                    })
                    .or_insert((y - 1, y + 1));
            }
        }

        for (&x, &(min_y, max_y)) in min_max_y.iter() {
            board.set_edge((x, min_y + 1), (x, min_y), (x, max_y - 1));
            board.set_edge((x, max_y - 1), (x, max_y), (x, min_y + 1));

            // update starting position to top left of reachable tiles
            if board.position.is_none()
                || min_y + 1 < board.position.unwrap().1
                || (x < board.position.unwrap().0 && min_y + 1 == board.position.unwrap().1)
            {
                board.position = Some((x, min_y + 1));
            }
        }

        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn building_board() {
        let raw_board = get_board();
        Board::from_str(raw_board).unwrap();
        Board::from_cubic_str(raw_board).unwrap();
    }

    #[test]
    fn valid_board_from_str() {
        let raw_board = get_board();
        let board = Board::from_str(raw_board).unwrap();

        let tile = &board.data[&4][&5];
        assert_eq!(tile, &Tile::Wall)
    }

    #[test]
    fn valid_board_from_cubic_str() {
        let raw_board = get_board();
        let board = Board::from_cubic_str(raw_board).unwrap();

        let tile = &board.data[&4][&5];
        assert_eq!(tile, &Tile::Wall)
    }

    #[test]
    fn valid_row_teleport_box() {
        let raw_board = get_board();
        let board = Board::from_str(raw_board).unwrap();

        let left = &board.edges[&(1, 6)][&(0, 6)];
        let right = &board.edges[&(12, 6)][&(13, 6)];
        assert_eq!(left, &(12, 6));
        assert_eq!(right, &(1, 6));
    }

    #[test]
    fn valid_col_teleport_box() {
        let raw_board = get_board();
        let board = Board::from_str(raw_board).unwrap();

        let up = &board.edges[&(11, 1)][&(11, 0)];
        let down = &board.edges[&(11, 12)][&(11, 13)];
        assert_eq!(up, &(11, 12));
        assert_eq!(down, &(11, 1));
    }

    #[test]
    fn valid_start_position() {
        let raw_board = get_board();
        let board = Board::from_str(raw_board).unwrap();

        assert_eq!(board.position, Some((9, 1)));
    }

    fn get_board() -> &'static str {
        r#"        ...#
    .#..
    #...
    ....
...#.......#
........#...
..#....#....
..........#.
    ...#....
    .....#..
    .#......
    ......#."#
    }
}
