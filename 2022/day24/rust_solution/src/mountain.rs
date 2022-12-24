use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

pub type Position = (usize, usize);
type Time = usize;

fn manhattan_distance(p1: Position, p2: Position) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

#[derive(Debug, Clone, Copy)]
enum Flow {
    Forward = 1,
    Backward = -1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Round {
    FirstPass,
    BackToSnack,
    LastPass,
}

#[derive(Debug, Clone, Copy)]
pub struct State {
    position: Position,
    elapsed: Time,
    round: Round,
}

impl State {
    fn new(position: Position) -> State {
        State {
            position,
            elapsed: 0,
            round: Round::FirstPass,
        }
    }

    pub fn start() -> State {
        State::new((0, 1))
    }

    pub fn next(&self, graph: &Graph) -> Vec<State> {
        static DIRS: [(isize, isize); 5] = [(1, 0), (-1, 0), (0, 1), (0, -1), (0, 0)];

        let mut next_states = vec![];
        for dir in DIRS {
            let (row, col) = (
                self.position.0 as isize + dir.0,
                self.position.1 as isize + dir.1,
            );

            if row < 0 || col < 0 {
                continue;
            }

            let position = (row as usize, col as usize);
            let elapsed = self.elapsed + 1;

            if !graph.occupied(elapsed, position) {
                let mut next = self.clone();
                next.position = position;
                next.elapsed = elapsed;
                next_states.push(next);
            }
        }

        next_states
    }
}

#[derive(Debug, Clone)]
pub struct Graph {
    blizzards_vertical: Vec<HashSet<Position>>,
    blizzards_horizontal: Vec<HashSet<Position>>,
}

impl Graph {
    fn new(
        blizzards_vertical: Vec<(Position, Flow)>,
        blizzards_horizontal: Vec<(Position, Flow)>,
        vertical_len: usize,
        horizontal_len: usize,
    ) -> Graph {
        let vertical_size = vertical_len - 2; // remove walls
        let blizzards_vertical = (0..(vertical_size))
            .map(|offset| {
                blizzards_vertical
                    .iter()
                    .map(|&(pos, flow)| {
                        let offset = flow as isize * offset as isize;
                        let col = (pos.1 as isize + offset).rem_euclid(vertical_size as isize);
                        let col = if col == 0 {
                            vertical_size
                        } else {
                            col as usize
                        };

                        (pos.0, col)
                    })
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();

        let horizontal_size = horizontal_len - 2; // remove walls
        let blizzards_horizontal = (0..(horizontal_size))
            .map(|offset| {
                blizzards_horizontal
                    .iter()
                    .map(|&(pos, flow)| {
                        let offset = flow as isize * offset as isize;
                        let row = (pos.0 as isize + offset).rem_euclid(horizontal_size as isize);
                        let row = if row == 0 {
                            horizontal_size
                        } else {
                            row as usize
                        };

                        (row, pos.1)
                    })
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();

        Graph {
            blizzards_vertical,
            blizzards_horizontal,
        }
    }

    fn occupied(&self, time: Time, position: Position) -> bool {
        let (h_off, v_off) = self.get_time_offset(time);

        if position == State::start().position || position == self.goal() {
            return false;
        }

        position.0 < 1
            || position.1 < 1
            || position.0 > self.blizzards_horizontal.len()
            || position.1 > self.blizzards_vertical.len()
            || self.blizzards_horizontal[h_off].contains(&position)
            || self.blizzards_vertical[v_off].contains(&position)
    }

    fn get_time_offset(&self, time: Time) -> (Time, Time) {
        (
            time % self.blizzards_horizontal.len(),
            time % self.blizzards_vertical.len(),
        )
    }

    fn goal(&self) -> Position {
        (
            self.blizzards_horizontal.len() + 1,
            self.blizzards_vertical.len(),
        )
    }

    fn can_reach_goal(&self, state: State, best_time: Time) -> bool {
        state.elapsed + manhattan_distance(state.position, self.goal()) < best_time
    }

    pub fn search(&self, start: State) -> Option<Time> {
        let mut visited = HashSet::new();
        let mut frontier = VecDeque::new();
        frontier.push_back(start.clone());

        let mut best_time: Option<Time> = None;
        while let Some(state) = frontier.pop_front() {
            let unique_view = (self.get_time_offset(state.elapsed), state.position);
            if !visited.insert(unique_view) {
                continue;
            }

            if let Some(time) = best_time {
                if !self.can_reach_goal(state, time) {
                    continue;
                }
            }

            if state.position == self.goal() {
                best_time = match best_time {
                    Some(time) => Some(time.min(state.elapsed)),
                    None => Some(state.elapsed),
                };
            }

            for state in state.next(&self) {
                frontier.push_back(state);
            }
        }

        best_time
    }

    pub fn search_circular(&self, start: State) -> Option<Time> {
        let mut visited = HashSet::new();
        let mut frontier = VecDeque::new();
        frontier.push_back(start.clone());

        let mut best_time: Option<Time> = None;
        while let Some(mut state) = frontier.pop_front() {
            let unique_view = (
                self.get_time_offset(state.elapsed),
                state.position,
                state.round,
            );
            if !visited.insert(unique_view) {
                continue;
            }

            if let Some(time) = best_time {
                if !self.can_reach_goal(state, time) {
                    continue;
                }
            }

            if state.round == Round::FirstPass && state.position == self.goal() {
                state.round = Round::BackToSnack;
            }

            if state.round == Round::BackToSnack && state.position == start.position {
                state.round = Round::LastPass;
            }

            if state.round == Round::LastPass && state.position == self.goal() {
                best_time = match best_time {
                    Some(time) => Some(time.min(state.elapsed)),
                    None => Some(state.elapsed),
                };
            }

            for state in state.next(&self) {
                frontier.push_back(state);
            }
        }

        best_time
    }
}

impl FromStr for Graph {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vertical = vec![];
        let mut horizontal = vec![];

        for (row, line) in s.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let position = (row, col);
                match ch {
                    '^' => horizontal.push((position, Flow::Backward)),
                    'v' => horizontal.push((position, Flow::Forward)),
                    '<' => vertical.push((position, Flow::Backward)),
                    '>' => vertical.push((position, Flow::Forward)),
                    '#' | '.' => continue,
                    _ => return Err("unknown character"),
                }
            }
        }

        let vertical_len = s.lines().take(1).collect::<Vec<_>>()[0].chars().count();
        let horizontal_len = s.lines().count();

        Ok(Graph::new(
            vertical,
            horizontal,
            vertical_len,
            horizontal_len,
        ))
    }
}
