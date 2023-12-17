use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    str::FromStr,
};

#[derive(thiserror::Error, Debug)]
pub enum CityErr {
    #[error("the map can not be empty")]
    Empty,

    #[error("the map contains a non numeric city block with value: {0}")]
    NonNumericBlock(char),

    #[error("not all rows of the map are of the same length")]
    UnEven,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Dir {
    fn apply(self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Self::Up => {
                if pos.1 == 0 {
                    None
                } else {
                    Some((pos.0, pos.1 - 1))
                }
            }
            Self::Left => {
                if pos.0 == 0 {
                    None
                } else {
                    Some((pos.0 - 1, pos.1))
                }
            }
            Self::Right => Some((pos.0 + 1, pos.1)),
            Self::Down => Some((pos.0, pos.1 + 1)),
        }
    }

    fn sides(self) -> [Dir; 2] {
        match self {
            Self::Down | Self::Up => [Self::Right, Self::Left],
            Self::Left | Self::Right => [Self::Down, Self::Up],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node {
    steps: u8,
    x: usize,
    y: usize,
    dir: Dir,
}

impl Node {
    fn next_nodes(
        self,
        data: &[Vec<u8>],
        reset_steps: u8,
        ultra: u8,
    ) -> [Option<(usize, Node)>; 3] {
        let out_of_bounds = |dest: (usize, usize)| dest.1 >= data.len() || dest.0 >= data[0].len();

        let forward = if self.steps > 0 {
            self.dir
                .apply((self.x, self.y))
                .and_then(|pos| if out_of_bounds(pos) { None } else { Some(pos) })
                .map(|pos| {
                    (
                        data[pos.1][pos.0] as usize,
                        Self {
                            x: pos.0,
                            y: pos.1,
                            steps: self.steps - 1,
                            ..self
                        },
                    )
                })
        } else {
            None
        };

        let sides = self.dir.sides().map(|dir| {
            let steps = ultra.max(1);

            let mut dest = dir.apply((self.x, self.y))?;
            if out_of_bounds(dest) {
                return None;
            }
            let mut cost = data[dest.1][dest.0] as usize;
            for _ in 1..steps {
                dest = dir.apply(dest)?;
                if out_of_bounds(dest) {
                    return None;
                }

                cost += data[dest.1][dest.0] as usize;
            }

            Some((
                cost,
                Self {
                    x: dest.0,
                    y: dest.1,
                    dir,
                    steps: reset_steps - steps,
                },
            ))
        });

        [forward, sides[0], sides[1]]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    data: Vec<Vec<u8>>,
}

impl Map {
    pub fn minimize_heat_loss(&self, max_steps_in_one_direction: u8, ultra: u8) -> usize {
        let mut frontier: BinaryHeap<Reverse<(usize, Node)>> = Default::default();
        let mut visited: HashSet<Node> = Default::default();

        let start = Node {
            steps: max_steps_in_one_direction,
            x: 0,
            y: 0,
            dir: Dir::Right,
        };

        frontier.push(Reverse((0, start)));
        while let Some(Reverse((cost, node))) = frontier.pop() {
            if !visited.insert(node) {
                continue;
            }

            if node.y == (self.data.len() - 1) && node.x == (self.data[0].len() - 1) {
                return cost;
            }

            for (enter_cost, next) in node
                .next_nodes(&self.data, max_steps_in_one_direction, ultra)
                .into_iter()
                .flatten()
            {
                frontier.push(Reverse((cost + enter_cost, next)))
            }
        }

        unreachable!()
    }
}

impl FromStr for Map {
    type Err = CityErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .trim()
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|ch| {
                        ch.to_digit(10)
                            .ok_or(CityErr::NonNumericBlock(ch))
                            .map(|num| num as u8)
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        if data.is_empty() {
            return Err(CityErr::Empty);
        }

        let rlen = data[0].len();
        if data.iter().any(|row| row.len() != rlen) {
            return Err(CityErr::UnEven);
        }

        Ok(Self { data })
    }
}

#[cfg(test)]
mod tests {
    use super::Map;

    #[test]
    fn parse_map() {
        let input = r#"2413
3215"#;
        let expected_output = Map {
            data: vec![vec![2, 4, 1, 3], vec![3, 2, 1, 5]],
        };

        let output: Map = input.parse().unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn minimize_heat_loss() {
        let map: Map = r#"2413
3215"#
            .parse()
            .unwrap();

        assert_eq!(map.minimize_heat_loss(3, 0), 11);
    }
}
