use std::str::FromStr;

use crate::position::{Direction, Position};

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

    #[error("bad rgb format ({0})")]
    BadRgb(String),

    #[error("bad steps value in rgb format ({0})")]
    BadStepsRgb(String),

    #[error("unknown direction in rgb format ({0})")]
    UnknownDirRgb(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    steps: usize,
    direction: Direction,
}

impl Instruction {
    pub fn from_rgb(s: &str) -> Result<Self, DiggerErr> {
        let parts = s.trim().split_ascii_whitespace().collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(DiggerErr::UnknownFormat(s.into()));
        }

        let color = parts[2]
            .trim()
            .strip_prefix('(')
            .and_then(|color| color.strip_prefix('#'))
            .and_then(|color| color.strip_suffix(')'))
            .ok_or_else(|| DiggerErr::UnknownColor(parts[3].into()))?;
        if color.as_bytes().len() != 6 {
            return Err(DiggerErr::BadRgb(color.into()));
        }

        let steps = &color[0..5];
        let steps =
            usize::from_str_radix(steps, 16).map_err(|_| DiggerErr::BadStepsRgb(steps.into()))?;
        let direction = color.chars().nth(5).unwrap();
        let direction =
            Direction::from_digit(direction).ok_or(DiggerErr::UnknownDirRgb(direction))?;

        Ok(Self { steps, direction })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    instructions: Vec<Instruction>,
}

impl Plan {
    pub fn calculate_area(&self) -> usize {
        let mut dots = Vec::new();
        let mut perimeter = 0;

        let mut pos = Position::default();
        for inst in self.instructions.iter() {
            perimeter += inst.steps;
            pos += inst.direction.into_step(inst.steps as isize);
            dots.push(pos);
        }

        let area = dots
            .windows(2)
            .map(|p| ((p[1].y + p[0].y) * (p[0].x - p[1].x)))
            .sum::<isize>();

        (area as usize) / 2 + perimeter / 2 + 1
    }

    pub fn from_rgb(s: &str) -> Result<Self, DiggerErr> {
        let instructions = s
            .trim()
            .lines()
            .map(Instruction::from_rgb)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { instructions })
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

        Ok(Self { steps, direction })
    }
}

#[cfg(test)]
mod tests {
    use super::{Direction, Instruction};

    #[test]
    fn parse_instruction() {
        let input = "R 6 (#70c710)";
        let expected_output = Instruction {
            steps: 6,
            direction: Direction::Right,
        };

        let output: Instruction = input.parse().unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn parse_from_rgb() {
        let input = "R 6 (#70c710)";
        let expected_output = Instruction {
            steps: 461937,
            direction: Direction::Right,
        };

        let output = Instruction::from_rgb(input).unwrap();
        assert_eq!(output, expected_output);
    }
}
