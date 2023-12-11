use std::collections::HashSet;

const GALAXY: char = '#';
const EMPTY: char = '.';

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn hamilton_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Universe {
    galaxies: Vec<Position>,
}

impl Universe {
    pub fn minimum_path_sum(&self) -> usize {
        self.galaxies
            .iter()
            .enumerate()
            .flat_map(|(i, g1)| {
                self.galaxies
                    .iter()
                    .skip(i + 1)
                    .map(|g2| g1.hamilton_distance(g2))
            })
            .sum::<usize>()
    }

    pub fn from_str(s: &str, empty_column_factor: usize) -> Option<Self> {
        let graph = s
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>();

        if graph.is_empty() {
            return None;
        }

        // find all empty rows/columns
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
        enum Type {
            Row,
            Col,
        }
        let mut empty: HashSet<(Type, usize)> = Default::default();
        // empty rows
        empty.extend(
            graph
                .iter()
                .enumerate()
                .filter(|(_, row)| row.iter().all(|ch| *ch == EMPTY))
                .map(|(rdx, _)| (Type::Row, rdx)),
        );
        // empty cols
        empty.extend(
            (0..graph[0].len())
                .filter(|cdx| (0..graph.len()).all(|rdx| graph[rdx][*cdx] == EMPTY))
                .map(|cdx| (Type::Col, cdx)),
        );

        let mut galaxies = vec![];
        let mut rdx_offset = 0;
        for (rdx, row) in graph.iter().enumerate() {
            if empty.contains(&(Type::Row, rdx)) {
                rdx_offset += 1;
                continue;
            }

            let mut cdx_offset = 0;
            for (cdx, ch) in row.iter().enumerate() {
                if empty.contains(&(Type::Col, cdx)) {
                    cdx_offset += 1;
                    continue;
                }

                if *ch == GALAXY {
                    galaxies.push(Position::new(
                        cdx + (cdx_offset * (empty_column_factor - 1)),
                        rdx + (rdx_offset * (empty_column_factor - 1)),
                    ));
                }
            }
        }

        Some(Self { galaxies })
    }
}

#[cfg(test)]
mod tests {
    use super::{Position, Universe};

    #[test]
    fn parse_universe() {
        let input = r#"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."#;
        let expected_output = Universe {
            galaxies: vec![
                Position::new(4, 0),
                Position::new(9, 1),
                Position::new(0, 2),
                Position::new(8, 5),
                Position::new(1, 6),
                Position::new(12, 7),
                Position::new(9, 10),
                Position::new(0, 11),
                Position::new(5, 11),
            ],
        };

        let output = Universe::from_str(input, 2).unwrap();
        assert_eq!(output, expected_output);
    }
}
