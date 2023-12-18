use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use crate::{
    position::{Position, STEPS},
    rgb::Rgb,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Terrain {
    edges: HashMap<Position, Rgb>,
    xs: Range<isize>,
    ys: Range<isize>,
}

impl Terrain {
    pub fn new() -> Self {
        Self {
            edges: Default::default(),
            xs: (0..0),
            ys: (0..0),
        }
    }

    pub fn dig(&mut self, position: Position, color: Rgb) {
        self.edges.insert(position, color);

        self.xs.start = self.xs.start.min(position.x);
        self.xs.end = self.xs.end.max(position.x + 1);
        self.ys.start = self.ys.start.min(position.y);
        self.ys.end = self.ys.end.max(position.y + 1);
    }

    pub fn dig_interior(&self) -> usize {
        let mut digged = 0;

        let mut previously_visited: HashSet<Position> = Default::default();
        for x in self.xs.clone() {
            for y in self.ys.clone() {
                let position = Position::new(x, y);
                if previously_visited.contains(&position) {
                    continue;
                }
                if self.edges.contains_key(&position) {
                    digged += 1;
                    continue;
                }

                let (inside, visited) = self.dig_interior_bfs(position, &previously_visited);
                if inside {
                    digged += visited.len();
                }
                previously_visited.extend(visited);
            }
        }

        digged
    }

    fn dig_interior_bfs(
        &self,
        start_position: Position,
        previously_visited: &HashSet<Position>,
    ) -> (bool, HashSet<Position>) {
        let mut visited: HashSet<Position> = Default::default();

        // we're in the inside of the shape
        // as long as we don't reach a corner
        let mut inside = true;
        let mut frontier = vec![start_position];
        while let Some(position) = frontier.pop() {
            if self.edges.contains_key(&position) || previously_visited.contains(&position) {
                continue;
            }

            if !visited.insert(position) {
                continue;
            }

            if position.x < self.xs.start
                || position.x >= self.xs.end
                || position.y < self.ys.start
                || position.y >= self.ys.end
            {
                // reached the "outside"
                inside = false;
                continue;
            }

            frontier.extend(
                STEPS
                    .iter()
                    .map(|step| position + *step)
                    .filter(|next| !visited.contains(next)),
            );
        }

        (inside, visited)
    }
}

impl Default for Terrain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{digger::Plan, position::Position, rgb::Rgb};

    use super::Terrain;

    #[test]
    fn test_dig() {
        let mut terrain = Terrain::default();
        terrain.dig(Position::new(0, 0), Rgb::black());
        terrain.dig(Position::new(0, 2), Rgb::black());
        terrain.dig(Position::new(1, 2), Rgb::black());
        terrain.dig(Position::new(5, 1), Rgb::black());

        assert_eq!(terrain.xs.start, 0);
        assert_eq!(terrain.xs.end, 6);
        assert_eq!(terrain.ys.start, 0);
        assert_eq!(terrain.ys.end, 3);
    }

    #[test]
    fn from_plan() {
        let plan: Plan = "R 6 (#001100)".parse().unwrap();
        let terrain = Terrain::from(plan);

        assert!(terrain.edges.contains_key(&Position::new(0, 0)));
        assert!(terrain.edges.contains_key(&Position::new(3, 0)));
        assert!(terrain.edges.contains_key(&Position::new(6, 0)));

        assert!(!terrain.edges.contains_key(&Position::new(-1, 0)));
        assert!(!terrain.edges.contains_key(&Position::new(6, 1)));
        assert!(!terrain.edges.contains_key(&Position::new(7, 0)));

        assert_eq!(terrain.xs, (0..7));
        assert_eq!(terrain.ys, (0..1));
    }

    #[test]
    fn from_big_plan() {
        let plan: Plan = r#"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"#
            .parse()
            .unwrap();
        let terrain = Terrain::from(plan);

        assert_eq!(terrain.edges.len(), 38);
    }

    #[test]
    fn dig_interior() {
        let plan: Plan = r#"R 2 (#001100)
D 2 (#001100)
L 2 (#001100)
U 2 (#001100)"#
            .parse()
            .unwrap();

        let terrain: Terrain = plan.into();
        assert_eq!(terrain.dig_interior(), 9);
    }
}
