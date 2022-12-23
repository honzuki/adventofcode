use std::{collections::HashSet, fmt::Display, hash::Hash, str::FromStr, vec};

type Position = (isize, isize);

#[derive(Debug, Clone)]
enum ProposeDir {
    North,
    South,
    West,
    East,
}

impl ProposeDir {
    pub fn new() -> Vec<ProposeDir> {
        vec![
            ProposeDir::North,
            ProposeDir::South,
            ProposeDir::West,
            ProposeDir::East,
        ]
    }

    pub fn get_offsets(&self) -> Vec<Position> {
        match self {
            ProposeDir::North => vec![(0, -1), (1, -1), (-1, -1)],
            ProposeDir::South => vec![(0, 1), (1, 1), (-1, 1)],
            ProposeDir::West => vec![(-1, 0), (-1, 1), (-1, -1)],
            ProposeDir::East => vec![(1, 0), (1, -1), (1, 1)],
        }
    }

    pub fn get_offset(&self) -> Position {
        match self {
            ProposeDir::North => (0, -1),
            ProposeDir::South => (0, 1),
            ProposeDir::West => (-1, 0),
            ProposeDir::East => (1, 0),
        }
    }

    pub fn rotate(list: &mut Vec<ProposeDir>) {
        let first = list.remove(0);
        list.push(first);
    }
}

#[derive(Debug, Clone, Copy)]
enum ProposeMove {
    Valid(Position, Position),
    Invalid(usize, Position),
}

impl ProposeMove {
    fn increase_value(&self) -> Self {
        match self {
            Self::Valid(_, pos) => Self::Invalid(1, *pos),
            Self::Invalid(count, pos) => Self::Invalid(*count + 1, *pos),
        }
    }
}

impl Hash for ProposeMove {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Valid(_, pos) | Self::Invalid(_, pos) => pos.hash(state),
        }
    }
}

impl Eq for ProposeMove {}
impl PartialEq for ProposeMove {
    /// 2 proposes are equal if they move to the same positions
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                ProposeMove::Valid(_, p1) | ProposeMove::Invalid(_, p1),
                ProposeMove::Valid(_, p2) | ProposeMove::Invalid(_, p2),
            ) if p1 == p2 => true,
            _ => false,
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[derive(Debug, Clone)]
pub struct GameManager {
    elves: HashSet<Position>,
    dir_list: Vec<ProposeDir>,
}

impl GameManager {
    fn new(elves: HashSet<Position>) -> GameManager {
        GameManager {
            elves,
            dir_list: ProposeDir::new(),
        }
    }

    pub fn containing_tiles(&self) -> usize {
        let (e1, e2) = self.bounding_box();

        let total_tiles = (e1.0.abs_diff(e2.0) + 1) * (e1.1.abs_diff(e2.1) + 1);
        let empty_tiles = total_tiles - self.elves.len();

        empty_tiles
    }

    // returns the amount of elves that moved during the round
    pub fn execute_round(&mut self) -> usize {
        let mut proposes: HashSet<ProposeMove> = HashSet::new();

        for &position in self.elves.iter() {
            if let Some(pm) = self.propose_move(position) {
                if let Some(&pm) = proposes.get(&pm) {
                    proposes.remove(&pm);
                    proposes.insert(pm.increase_value());
                } else {
                    proposes.insert(pm);
                }
            }
        }

        let mut move_count = 0;
        for propose in proposes {
            match propose {
                ProposeMove::Valid(from, to) => {
                    self.elves.remove(&from);
                    self.elves.insert(to);

                    move_count += 1;
                }
                ProposeMove::Invalid(_, _) => continue,
            }
        }

        ProposeDir::rotate(&mut self.dir_list);

        move_count
    }

    fn propose_move(&self, position: Position) -> Option<ProposeMove> {
        if !self.should_move(position) {
            return None;
        }

        for dir in self.dir_list.iter() {
            let contain_elves = dir.get_offsets().iter().any(|offset| {
                self.elves
                    .contains(&(position.0 + offset.0, position.1 + offset.1))
            });

            if !contain_elves {
                let offset = dir.get_offset();
                return Some(ProposeMove::Valid(
                    position,
                    (position.0 + offset.0, position.1 + offset.1),
                ));
            }
        }

        None
    }

    fn should_move(&self, position: Position) -> bool {
        (-1..=1isize)
            .flat_map(|r| (-1..=1isize).map(|c| (r, c)).collect::<Vec<Position>>())
            .collect::<Vec<Position>>()
            .iter()
            .filter(|&&offset| offset != (0, 0))
            .any(|offset| {
                self.elves
                    .contains(&(position.0 + offset.0, position.1 + offset.1))
            })
    }

    fn bounding_box(&self) -> (Position, Position) {
        let &pos = self.elves.iter().take(1).collect::<Vec<&Position>>()[0];
        let mut min_x = pos.0;
        let mut max_x = pos.0;
        let mut min_y = pos.1;
        let mut max_y = pos.1;

        for &pos in self.elves.iter() {
            min_x = min_x.min(pos.0);
            max_x = max_x.max(pos.0);
            min_y = min_y.min(pos.1);
            max_y = max_y.max(pos.1);
        }

        ((min_x, min_y), (max_x, max_y))
    }
}

impl FromStr for GameManager {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elves = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, ch)| {
                        if ch == '#' {
                            Some((x as isize, y as isize))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(GameManager::new(elves))
    }
}

impl Display for GameManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (e1, e2) = self.bounding_box();

        let tiles = (e1.1..=e2.1)
            .map(|y| {
                (e1.0..=e2.0)
                    .map(|x| {
                        if self.elves.contains(&(x, y)) {
                            '#'
                        } else {
                            '.'
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>();

        write!(f, "{}", tiles.join("\n"))
    }
}
