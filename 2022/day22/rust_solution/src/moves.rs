use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Movements(Vec<Movement>);

impl Movements {
    pub fn get_moves(&self) -> &[Movement] {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub enum Movement {
    RotateLeft,
    RotateRight,
    Step(usize),
}

impl FromStr for Movements {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut moves = Vec::new();

        // assuming that s is ascii only
        let mut seq_start = 0;
        for (idx, ch) in s.chars().enumerate() {
            match ch {
                'R' => {
                    let steps = &s[seq_start..idx];
                    let steps: usize = steps
                        .parse()
                        .map_err(|_| "failed to parse the num of steps")?;

                    seq_start = idx + 1;
                    moves.push(Movement::Step(steps));
                    moves.push(Movement::RotateRight);
                }
                'L' => {
                    let steps = &s[seq_start..idx];
                    let steps: usize = steps
                        .parse()
                        .map_err(|_| "failed to parse the num of steps")?;

                    seq_start = idx + 1;
                    moves.push(Movement::Step(steps));
                    moves.push(Movement::RotateLeft);
                }
                _ if ch.is_numeric() => {}
                _ => return Err("unknown pattern"),
            };
        }

        if seq_start < s.len() {
            let steps = &s[seq_start..s.len()];
            let steps: usize = steps
                .parse()
                .map_err(|_| "failed to parse the num of steps")?;

            moves.push(Movement::Step(steps));
        }

        Ok(Movements(moves))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_pattern() {
        let pat = get_pattern();
        Movements::from_str(pat).unwrap();
    }

    fn get_pattern() -> &'static str {
        r#"10R5L5R10L4R5L5"#
    }
}
