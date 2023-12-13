use std::str::FromStr;

const ASH: char = '.';
const ROCK: char = '#';

#[derive(thiserror::Error, Debug)]
pub enum PatternErr {
    #[error("unknown pattern type: {0}")]
    UnknownType(char),

    #[error("the pattern can not be empty")]
    Empty,

    #[error("not all of the rows of the pattern are of the same length")]
    NotEven,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Ash,
    Rock,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern {
    data: Vec<Vec<Type>>,
}

impl Pattern {
    pub fn find_vertical_reflection(&self, smudge: bool) -> Option<usize> {
        let view = PatternView::new_vertical(self);

        if smudge {
            Self::find_reflection_with_smudge(view)
        } else {
            Self::find_reflection(view)
        }
    }

    pub fn find_horizontal_reflection(&self, smudge: bool) -> Option<usize> {
        let view = PatternView::new_horizontal(self);

        if smudge {
            Self::find_reflection_with_smudge(view)
        } else {
            Self::find_reflection(view)
        }
    }

    fn find_reflection(view: PatternView) -> Option<usize> {
        // for each possible start of reflection
        for idx in 0..(view.len() - 1) {
            let mut left = idx;
            let mut right = idx + 1;

            // compare every pair of lines until you
            // reach the edge on either of the sides
            while view.compare(left, right) {
                if left == 0 || right == (view.len() - 1) {
                    return Some(idx);
                } else {
                    left -= 1;
                    right += 1;
                }
            }
        }

        None
    }

    fn find_reflection_with_smudge(view: PatternView) -> Option<usize> {
        // for each possible start of reflection
        for idx in 0..(view.len() - 1) {
            let mut left = idx;
            let mut right = idx + 1;
            let mut error_count = 0;

            // compare every other pair of lines until you
            // reach the edge on either of the sides, while
            // requiring exactly 1 error
            loop {
                error_count += view.count_compare(left, right);
                if error_count > 1 {
                    break;
                }

                if left == 0 || right == (view.len() - 1) {
                    if error_count == 1 {
                        return Some(idx);
                    } else {
                        break;
                    }
                } else {
                    left -= 1;
                    right += 1;
                }
            }
        }

        None
    }
}

impl FromStr for Pattern {
    type Err = PatternErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .trim()
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|ch| match ch {
                        ASH => Ok(Type::Ash),
                        ROCK => Ok(Type::Rock),
                        _ => Err(PatternErr::UnknownType(ch)),
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        if data.is_empty() {
            return Err(PatternErr::Empty);
        }

        if data.iter().any(|row| row.len() != data[0].len()) {
            return Err(PatternErr::NotEven);
        }

        Ok(Self { data })
    }
}

// Abstract whether we're going over the rows
// or the columns, to allow for a single
// algorithm that can check for both directions
struct PatternView<'a> {
    data: &'a [Vec<Type>],
    vertical: bool,
}

impl<'a> PatternView<'a> {
    fn new_horizontal(pattern: &'a Pattern) -> Self {
        Self {
            data: &pattern.data,
            vertical: false,
        }
    }

    fn new_vertical(pattern: &'a Pattern) -> Self {
        Self {
            data: &pattern.data,
            vertical: true,
        }
    }

    fn len(&self) -> usize {
        match self.vertical {
            true => self.data[0].len(),
            false => self.data.len(),
        }
    }

    fn compare(&self, l: usize, r: usize) -> bool {
        match self.vertical {
            true => self.data.iter().all(|row| row[l] == row[r]),
            false => self.data[l] == self.data[r],
        }
    }

    fn count_compare(&self, l: usize, r: usize) -> usize {
        match self.vertical {
            true => self.data.iter().filter(|row| row[l] != row[r]).count(),
            false => self.data[l]
                .iter()
                .zip(self.data[r].iter())
                .filter(|(l, r)| l != r)
                .count(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Pattern, Type};

    #[test]
    fn parse_pattern() {
        let input = r#"#.##..##.
..#.##.#."#;
        let expected_output = Pattern {
            data: vec![
                vec![
                    Type::Rock,
                    Type::Ash,
                    Type::Rock,
                    Type::Rock,
                    Type::Ash,
                    Type::Ash,
                    Type::Rock,
                    Type::Rock,
                    Type::Ash,
                ],
                vec![
                    Type::Ash,
                    Type::Ash,
                    Type::Rock,
                    Type::Ash,
                    Type::Rock,
                    Type::Rock,
                    Type::Ash,
                    Type::Rock,
                    Type::Ash,
                ],
            ],
        };

        let output: Pattern = input.parse().unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn vertical_pattern() {
        let input: Pattern = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#
            .parse()
            .unwrap();
        let expected_output = 4;

        let output = input.find_vertical_reflection(false).unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn horizontal_pattern() {
        let input: Pattern = r#"#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"#
            .parse()
            .unwrap();
        let expected_output = 3;

        let output = input.find_horizontal_reflection(false).unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn horizontal_pattern_with_smudge() {
        let input: Pattern = r#"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."#
            .parse()
            .unwrap();
        let expected_output = 2;

        let output = input.find_horizontal_reflection(true).unwrap();
        assert_eq!(output, expected_output);
    }
}
