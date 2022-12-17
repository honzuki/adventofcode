use std::{collections::HashSet, fmt::Display, str::FromStr};

pub mod tiles;
use tiles::Tile;

mod jet;

#[derive(Debug, Clone)]
pub struct GameManager {
    board: Vec<HashSet<usize>>,
    tiles: Vec<Tile>,
    tile_index: usize,
    tile_position: Option<(usize, usize)>, // bottom-left corner
    jet_pattern: jet::Pattern,
    height: usize,
}

impl GameManager {
    pub fn new(width: usize, tiles: Vec<Tile>, jet_pattern: &str) -> GameManager {
        let mut floor = HashSet::new();
        floor.insert(0);

        GameManager {
            board: vec![floor.clone(); width],
            tiles,
            tile_index: 0,
            tile_position: None,
            jet_pattern: jet::Pattern::from_str(jet_pattern).unwrap(),
            height: 0,
        }
    }

    // returns wether the current tile reached it final position
    pub fn tick(&mut self) -> bool {
        let tile = &self.tiles[self.tile_index];

        let (mut x, y) = match self.tile_position {
            None => {
                // place the new tiles
                let position = (2, self.height + 4);
                self.tile_position = Some(position);

                position
            }
            Some(position) => position,
        };

        // push according to the jet pattern
        let new_x = match self.jet_pattern.fetch() {
            jet::Push::Left => match x > 0 {
                false => 0,
                true => x - 1,
            },
            jet::Push::Right => x + 1,
        };
        if self.collide_count(tile, (new_x, y)) == 0 {
            x = new_x;
        }

        // push down
        assert!(y > 0); // we have the floor at y=0
        match self.collide_count(tile, (x, y - 1)) {
            0 => {
                self.tile_position = Some((x, y - 1));
                false
            }
            _ => {
                // the tile reached its final place
                self.place_current_tile((x, y));
                true
            }
        }
    }

    pub fn find_cycle(&mut self) -> Option<(usize, usize)> {
        let cmp_size = 50;

        let try_size = self.jet_pattern.size();
        let try_count = if try_size < 500 { 50 } else { 3 };
        let attempts = (0..(try_size * try_count))
            .map(|block_count| {
                while !self.tick() {}

                // collect the last 'cmp_size' lines
                let board = self
                    .board
                    .iter()
                    .map(|set| {
                        (0..cmp_size)
                            .map(|y_diff| set.contains(&(self.height - y_diff)))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
                (
                    block_count,
                    (board, self.tile_index, self.jet_pattern.position()),
                )
            })
            .collect::<Vec<_>>();

        for idx1 in 0..(attempts.len() - try_size) {
            for idx2 in (idx1 + try_size + 1)..(attempts.len() - try_size) {
                if attempts[idx1].1 != attempts[idx2].1 {
                    continue; // fast return
                }

                let equal_count = (0..try_size)
                    .filter(|idx| attempts[idx1 + idx].1 == attempts[idx2 + idx].1)
                    .count();
                if equal_count == try_size {
                    return Some((attempts[idx1].0, attempts[idx2].0));
                }
            }
        }

        None
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    fn is_occupied(&self, x: usize, y: usize) -> bool {
        if x >= self.board.len() {
            return true;
        }

        self.board[x].contains(&y)
    }

    fn collide_count(&self, tile: &Tile, position: (usize, usize)) -> usize {
        tile.as_raw()
            .iter()
            .enumerate()
            .map(|(y_dist, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x_dist, active)| {
                        *active && self.is_occupied(position.0 + x_dist, position.1 + y_dist)
                    })
                    .filter(|collide| *collide)
                    .count()
            })
            .sum::<usize>()
    }

    fn place_current_tile(&mut self, position: (usize, usize)) {
        let tile = &self.tiles[self.tile_index];

        for (y_dist, row) in tile.as_raw().iter().enumerate() {
            for (x_dist, active) in row.iter().enumerate() {
                if *active {
                    let x_pos = position.0 + x_dist;
                    let y_pos = position.1 + y_dist;
                    self.board[x_pos].insert(y_pos);

                    self.height = self.height.max(y_pos);
                }
            }
        }

        self.tile_index = (self.tile_index + 1) % self.tiles.len();
        self.tile_position = None;
    }
}

impl Display for GameManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = Vec::with_capacity(self.height);

        for y in 0..self.height {
            let mut row = String::with_capacity(self.board.len());
            row.push('|');
            for x in 0..self.board.len() {
                row.push(match self.board[x].contains(&y) {
                    true => '#',
                    false => '.',
                });
            }
            row.push('|');
            board.push(row);
        }

        board = board.into_iter().rev().collect::<Vec<_>>();
        f.write_fmt(format_args!("{}", board.join("\n")))
    }
}
