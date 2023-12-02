use std::{iter, ops::Add, str::FromStr};

const KNOWN_COLORS: [&str; 3] = ["red", "green", "blue"];

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct Set {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Set {
    // returns true if 'self' can not fit inside 'max_set'
    pub fn overflow(&self, max_set: &Self) -> bool {
        self.blue > max_set.blue || self.green > max_set.green || self.red > max_set.red
    }

    pub fn power(&self) -> u64 {
        self.red as u64 * self.green as u64 * self.blue as u64
    }

    fn from_parts(color: &str, count: u32) -> Result<Self, &'static str> {
        let set = match color {
            "red" => Self {
                red: count,
                ..Default::default()
            },
            "green" => Self {
                green: count,
                ..Default::default()
            },
            "blue" => Self {
                blue: count,
                ..Default::default()
            },
            _ => return Err("unknown color"),
        };

        Ok(set)
    }
}

impl Add for Set {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.red += rhs.red;
        self.blue += rhs.blue;
        self.green += rhs.green;
        self
    }
}

impl iter::Sum for Set {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Set::default(), |set, item| set + item)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub id: u32,
    pub sets: Vec<Set>,
}

impl Record {
    /// returns true if any of the sets overflow the 'max_set'
    pub fn overflow(&self, max_set: &Set) -> bool {
        self.sets.iter().any(|set| set.overflow(max_set))
    }

    /// calculates the minimum max_set that would satisfy this record
    pub fn find_max_set(&self) -> Set {
        self.sets
            .iter()
            .fold(Set::default(), |max_set: Set, set| Set {
                red: max_set.red.max(set.red),
                green: max_set.green.max(set.green),
                blue: max_set.blue.max(set.blue),
            })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ParseRecordErr {
    #[error("unknown record format")]
    UnknownFormat,

    #[error("the game id is not an integer")]
    BadId,

    #[error("the record contains an unknown color: {0}")]
    UnknownColor(String),
}

impl FromStr for Record {
    type Err = ParseRecordErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, data) = s
            .trim()
            .split_once(':')
            .ok_or(ParseRecordErr::UnknownFormat)?;

        // Parse game id
        let id = id.split_ascii_whitespace().collect::<Vec<_>>();
        if id.len() != 2 {
            return Err(ParseRecordErr::UnknownFormat);
        }
        let id = id[1].parse::<u32>().map_err(|_| ParseRecordErr::BadId)?;

        // Parse set list
        let sets = data
            .split(';')
            // map each set into list of (color, count)
            .map(|set| {
                set.split(',')
                    .map(|ball| {
                        let ball = ball.trim().split_ascii_whitespace().collect::<Vec<_>>();
                        if ball.len() != 2 {
                            return Err(ParseRecordErr::UnknownFormat);
                        }

                        let count = ball[0]
                            .parse::<u32>()
                            .map_err(|_| ParseRecordErr::UnknownFormat)?;
                        let color = ball[1].trim().to_lowercase();
                        if !KNOWN_COLORS.contains(&color.as_str()) {
                            return Err(ParseRecordErr::UnknownColor(color));
                        }

                        Ok((color, count))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            // collect each set into a Set
            .map(|sets| {
                let sets = sets?;
                let set = sets
                    .into_iter()
                    .map(|(color, count)| Set::from_parts(&color, count).unwrap())
                    .sum();

                Ok(set)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { id, sets })
    }
}

#[cfg(test)]
mod tests {
    use super::{Record, Set};

    #[test]
    fn parse_record_correctly() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected_output = Record {
            id: 1,
            sets: vec![
                Set {
                    blue: 3,
                    red: 4,
                    ..Default::default()
                },
                Set {
                    red: 1,
                    green: 2,
                    blue: 6,
                },
                Set {
                    green: 2,
                    ..Default::default()
                },
            ],
        };

        let output: Record = input.parse().expect("parse valid game record");
        assert_eq!(output, expected_output);
    }

    #[test]
    fn calculate_max_set() {
        let input: Record = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            .parse()
            .unwrap();
        let expected_output = Set {
            red: 4,
            green: 2,
            blue: 6,
        };

        assert_eq!(input.find_max_set(), expected_output);
    }
}
