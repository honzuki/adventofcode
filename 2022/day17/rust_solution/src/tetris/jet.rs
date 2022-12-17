use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Push {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Pattern {
    moves: Vec<Push>,
    current_index: usize,
}

impl Pattern {
    // returns the current push direction
    // and moves the pointer to the next one
    pub fn fetch(&mut self) -> Push {
        let push = self.moves[self.current_index].clone();
        self.current_index = (self.current_index + 1) % self.moves.len();

        push
    }

    pub fn position(&self) -> usize {
        self.current_index
    }

    pub fn size(&self) -> usize {
        self.moves.len()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PatternError {
    UnknownChar(char),
}

impl FromStr for Pattern {
    type Err = PatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moves = s
            .chars()
            .filter_map(|ch| match ch {
                '>' => Some(Ok(Push::Right)),
                '<' => Some(Ok(Push::Left)),
                '\n' => None,
                _ => Some(Err(PatternError::UnknownChar(ch))),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Pattern {
            moves,
            current_index: 0,
        })
    }
}
