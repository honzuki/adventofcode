use std::{
    collections::{BTreeSet, HashMap},
    str::FromStr,
};

#[derive(Debug, Clone)]
pub struct CaveMap {
    data: HashMap<isize, BTreeSet<isize>>,
    deepest: isize,
}

impl CaveMap {
    pub fn new() -> CaveMap {
        CaveMap {
            data: HashMap::new(),
            deepest: 0,
        }
    }

    pub fn add_block(&mut self, x: isize, y: isize, floor: bool) {
        self.data
            .entry(x)
            .and_modify(|bset| {
                bset.insert(y);
            })
            .or_insert({
                let mut new_bset = BTreeSet::new();
                new_bset.insert(y);
                new_bset
            });

        if floor {
            self.deepest = self.deepest.max(y);
        }
    }

    fn check_block(&mut self, x: isize, y: isize, has_floor: bool) -> Option<bool> {
        if has_floor {
            self.add_block(x, self.deepest + 2, false);
        }

        let rock_tree = self.data.get(&x)?;
        Some(rock_tree.contains(&y))
    }

    // returns true if the sand come to rest
    pub fn drop_sand(&mut self, start_x: isize, start_y: isize, has_floor: bool) -> bool {
        let mut x = start_x;
        let mut y = start_y;

        'main_loop: loop {
            match self.data.get(&x) {
                Some(rock_tree) => {
                    for rock_y in rock_tree {
                        if *rock_y < y {
                            continue;
                        } else if *rock_y == y {
                            return false;
                        } else if *rock_y == (y + 1) {
                            // try the left size
                            match self.check_block(x - 1, y + 1, has_floor) {
                                Some(true) => {
                                    // try the right side
                                    match self.check_block(x + 1, y + 1, has_floor) {
                                        Some(true) => {
                                            // we can't move to either of the sides, stop here
                                            self.add_block(x, y, false);
                                            return true;
                                        }
                                        Some(false) => {
                                            // move right
                                            x += 1;
                                            y += 1;
                                        }
                                        None => match has_floor {
                                            false => return false,
                                            true => self.add_block(x + 1, self.deepest + 2, false),
                                        },
                                    }
                                }
                                Some(false) => {
                                    // move left
                                    x -= 1;
                                    y += 1;
                                }
                                None => match has_floor {
                                    false => return false,
                                    true => self.add_block(x - 1, self.deepest + 2, false),
                                },
                            }

                            continue 'main_loop;
                        } else {
                            y = rock_y - 1;
                            continue 'main_loop;
                        }
                    }

                    match has_floor {
                        false => return false,
                        true => self.add_block(x, self.deepest + 2, false),
                    }
                }
                None => match has_floor {
                    false => return false,
                    true => self.add_block(x, self.deepest + 2, false),
                },
            }
        }
    }
}

impl FromStr for CaveMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cave_map = CaveMap::new();

        for block_lines in s.lines() {
            let mut last_block: Option<(isize, isize)> = None;

            for block in block_lines.split("->") {
                let (x, y) = block.split_once(',').unwrap();
                let x: isize = x.trim().parse().unwrap();
                let y: isize = y.trim().parse().unwrap();

                match last_block {
                    Some((last_x, last_y)) => {
                        for path_x in last_x.min(x)..=last_x.max(x) {
                            for path_y in last_y.min(y)..=last_y.max(y) {
                                cave_map.add_block(path_x, path_y, true);
                            }
                        }
                    }

                    None => cave_map.add_block(x, y, true),
                }

                last_block = Some((x, y));
            }
        }

        Ok(cave_map)
    }
}
