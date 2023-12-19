use std::{ops::Range, str::FromStr};

use once_cell::sync::Lazy;
use regex::Regex;

#[derive(thiserror::Error, Debug)]
pub enum PartErr {
    #[error("unknown part format")]
    UnknownFormat,

    #[error("bad '{0}' rating")]
    BadRating(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Part {
    pub x: u32,
    pub m: u32,
    pub a: u32,
    pub s: u32,
}

impl Part {
    pub fn sum_all(self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = PartErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap());

        let ratings = RE.captures(s.trim()).ok_or(PartErr::UnknownFormat)?;
        macro_rules! get_rating {
            ($num:literal, $tag:literal) => {{
                ratings
                    .get($num)
                    .unwrap()
                    .as_str()
                    .parse()
                    .map_err(|_| PartErr::BadRating($tag))?
            }};
        }

        Ok(Self {
            x: get_rating!(1, 'x'),
            m: get_rating!(2, 'm'),
            a: get_rating!(3, 'a'),
            s: get_rating!(4, 's'),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PartRange {
    pub x: Range<u32>,
    pub m: Range<u32>,
    pub a: Range<u32>,
    pub s: Range<u32>,
}

impl PartRange {
    pub fn new() -> Self {
        Self {
            x: (1..4001),
            m: (1..4001),
            a: (1..4001),
            s: (1..4001),
        }
    }

    pub fn size(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

impl Default for PartRange {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Part;

    #[test]
    fn part_part() {
        let input = r#"{x=787,m=2655,a=1222,s=2876}"#;
        let expected_output = Part {
            x: 787,
            m: 2655,
            a: 1222,
            s: 2876,
        };

        let output: Part = input.parse().unwrap();
        assert_eq!(output, expected_output);
    }
}
