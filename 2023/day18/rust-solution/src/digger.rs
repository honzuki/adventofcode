use std::str::FromStr;

use crate::{
    position::{Direction, Position},
    rgb::{Rgb, RgbErr},
    terrain::Terrain,
};

#[derive(thiserror::Error, Debug)]
pub enum DiggerErr {
    #[error("failed to parse instruction: {0}")]
    UnknownFormat(String),

    #[error("unknown direction value: {0}")]
    BadDirection(String),

    #[error("failed to parse number of steps ({0})")]
    BadSteps(String),

    #[error("unknown color format ({0})")]
    UnknownColor(String),

    #[error("{0}")]
    Rgb(#[from] RgbErr),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    steps: usize,
    color: Rgb,
    direction: Direction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    instructions: Vec<Instruction>,
}

impl From<Plan> for Terrain {
    fn from(plan: Plan) -> Self {
        let mut terrain = Terrain::default();

        let mut position = Position::default();
        terrain.dig(position, Rgb::black());
        for inst in plan.instructions {
            let step = inst.direction.into_step();
            for _ in 0..inst.steps {
                position += step;
                terrain.dig(position, inst.color);
            }
        }

        terrain
    }
}

impl FromStr for Plan {
    type Err = DiggerErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .trim()
            .lines()
            .map(|line| line.parse::<Instruction>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { instructions })
    }
}

impl FromStr for Instruction {
    type Err = DiggerErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.trim().split_ascii_whitespace().collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(DiggerErr::UnknownFormat(s.into()));
        }

        let direction = parts[0];
        let direction = match direction {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => return Err(DiggerErr::BadDirection(direction.into())),
        };

        let steps = parts[1]
            .parse::<usize>()
            .map_err(|_| DiggerErr::BadSteps(parts[1].into()))?;

        let color = parts[2]
            .trim()
            .strip_prefix('(')
            .and_then(|color| color.strip_suffix(')'))
            .ok_or_else(|| DiggerErr::UnknownColor(parts[3].into()))?;
        let color: Rgb = color.parse()?;

        Ok(Self {
            steps,
            color,
            direction,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Direction, Instruction};

    #[test]
    fn parse_instruction() {
        let input = "R 6 (#70c710)";
        let expected_output = Instruction {
            color: "#70c710".parse().unwrap(),
            steps: 6,
            direction: Direction::Right,
        };

        let output: Instruction = input.parse().unwrap();
        assert_eq!(output, expected_output);
    }
}
