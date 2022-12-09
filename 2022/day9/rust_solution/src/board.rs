use std::collections::hash_set::HashSet;

use crate::command::Cmd;

pub struct Board {
    head_pos: (isize, isize),
    tails_pos: Vec<(isize, isize)>,
    visited: HashSet<(isize, isize)>,
}

impl Board {
    pub fn new(tail_len: usize) -> Board {
        let mut visited = HashSet::new();
        visited.insert((0, 0));

        Board {
            head_pos: (0, 0),
            tails_pos: vec![(0, 0); tail_len],
            visited,
        }
    }

    pub fn execute(&mut self, cmd: &Cmd) {
        match cmd {
            Cmd::Left(count) => {
                for _ in 0..*count {
                    self.head_pos.0 -= 1;
                    self.update_tails();
                }
            }
            Cmd::Right(count) => {
                for _ in 0..*count {
                    self.head_pos.0 += 1;
                    self.update_tails();
                }
            }
            Cmd::Up(count) => {
                for _ in 0..*count {
                    self.head_pos.1 += 1;
                    self.update_tails();
                }
            }
            Cmd::Down(count) => {
                for _ in 0..*count {
                    self.head_pos.1 -= 1;
                    self.update_tails();
                }
            }
        }
    }

    pub fn update_tails(&mut self) {
        let mut followed = self.head_pos;
        for tail in self.tails_pos.iter_mut() {
            let new_pos = follow(*tail, followed);
            if new_pos == *tail {
                break;
            }

            *tail = new_pos;
            followed = new_pos;
        }

        self.visited
            .insert(self.tails_pos[self.tails_pos.len() - 1]);
    }

    pub fn get_visited_size(&self) -> usize {
        self.visited.len()
    }
}

fn follow(tail: (isize, isize), head: (isize, isize)) -> (isize, isize) {
    let x_diff = head.0 - tail.0;
    let x_adj = if x_diff < 0 { x_diff + 1 } else { x_diff - 1 };
    let y_diff = head.1 - tail.1;
    let y_adj = if y_diff < 0 { y_diff + 1 } else { y_diff - 1 };

    // moving in 1 coordinate only
    if x_diff.abs() > 1 && y_diff == 0 {
        return (tail.0 + x_adj, tail.1);
    }
    if x_diff == 0 && y_diff.abs() > 1 {
        return (tail.0, tail.1 + y_adj);
    }

    // diagonal adjustments
    if x_diff.abs() > 1 && y_diff.abs() == 1 {
        return (tail.0 + x_adj, head.1);
    }
    if x_diff.abs() == 1 && y_diff.abs() > 1 {
        return (head.0, tail.1 + y_adj);
    }
    if x_diff.abs() == 2 && y_diff.abs() == 2 {
        return (tail.0 + x_adj, tail.1 + y_adj);
    }

    tail
}
