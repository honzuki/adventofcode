use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Right,
    Left,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern {
    moves: Vec<Move>,
    current: usize,
}

impl Pattern {
    fn new(moves: Vec<Move>) -> Self {
        Self { moves, current: 0 }
    }
}

impl Iterator for Pattern {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.moves[self.current];
        self.current = (self.current + 1) % self.moves.len();

        Some(item)
    }
}

impl FromStr for Pattern {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s
            .trim()
            .chars()
            .map(|ch| match ch {
                'R' => Ok(Move::Right),
                'L' => Ok(Move::Left),
                _ => Err(format!("unknown move {}", ch)),
            })
            .collect::<Result<Vec<_>, _>>()?;

        if moves.is_empty() {
            return Err("empty pattern".into());
        }

        Ok(Self::new(moves))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub name: String,
    pub left: String,
    pub right: String,
}

pub fn parse_network(raw: &str) -> Result<HashMap<String, Node>, &'static str> {
    raw.trim()
        .lines()
        .map(|line| {
            let (name, dest) = line.split_once('=')?;
            let dest = dest.trim().strip_prefix('(').unwrap_or(dest);
            let dest = dest.strip_suffix(')').unwrap_or(dest);
            let (left, right) = dest.split_once(',')?;
            let node = Node {
                name: name.trim().to_string(),
                left: left.trim().to_string(),
                right: right.trim().to_string(),
            };

            Some((node.name.clone(), node))
        })
        .collect::<Option<HashMap<_, _>>>()
        .ok_or("unknown network format")
}

pub fn parse_input(input: &str) -> Result<(HashMap<String, Node>, Pattern), String> {
    let (pattern, network) = input
        .split_once("\n\n")
        .ok_or("unknown input format".to_string())?;

    let pattern: Pattern = pattern.parse()?;
    let network = parse_network(network)?;

    Ok((network, pattern))
}

#[cfg(test)]
mod tests {
    use crate::map::Node;

    use super::{Move, Pattern};

    #[test]
    fn parse_network() {
        let input = r#"AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#;
        let expected_output = [
            Node {
                name: "AAA".into(),
                left: "BBB".into(),
                right: "BBB".into(),
            },
            Node {
                name: "BBB".into(),
                left: "AAA".into(),
                right: "ZZZ".into(),
            },
            Node {
                name: "ZZZ".into(),
                left: "ZZZ".into(),
                right: "ZZZ".into(),
            },
        ];
        let output = super::parse_network(input).unwrap();

        for node in expected_output {
            assert_eq!(output[&node.name], node);
        }
    }

    #[test]
    fn parse_pattern() {
        let input = "LLR";
        let expected_output = Pattern::new(vec![Move::Left, Move::Left, Move::Right]);

        let output: Pattern = input.parse().unwrap();
        assert_eq!(output, expected_output);
    }
}
