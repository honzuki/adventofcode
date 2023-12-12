use std::{collections::HashMap, str::FromStr};

const OPERATIONAL: char = '.';
const DAMAGED: char = '#';
const UNKNOWN: char = '?';

#[derive(thiserror::Error, Debug)]
pub enum SpringErr {
    #[error("bad record format")]
    BadFormat,

    #[error("expected a list of sizes of each group, seperated by ','")]
    UnknownSizeFormat,

    #[error("unknown string condition: {0}")]
    UnknownStringCodition(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pattern: Vec<Spring>,
    groups: Vec<usize>,
}

impl Record {
    pub fn unfold(&mut self, size: usize) {
        self.pattern.push(Spring::Unknown);
        let pattern_length = self.pattern.len() * size - 1;
        self.pattern = std::mem::take(&mut self.pattern)
            .into_iter()
            .cycle()
            .take(pattern_length)
            .collect();

        let group_length = self.groups.len() * size;
        self.groups = std::mem::take(&mut self.groups)
            .into_iter()
            .cycle()
            .take(group_length)
            .collect();
    }

    pub fn count_arrangements(&self) -> usize {
        let last_damaged = self
            .pattern
            .iter()
            .enumerate()
            .rev()
            .find(|(_, spring)| matches!(spring, Spring::Damaged))
            .map(|(idx, _)| idx)
            .unwrap_or(0);

        let mut mem = HashMap::new();
        self.count_arrangements_rec(0, 0, last_damaged, &mut mem)
    }

    fn count_arrangements_rec(
        &self,
        start_pidx: usize,
        gidx: usize,
        last_damaged: usize,
        mem: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(count) = mem.get(&(start_pidx, gidx)) {
            return *count;
        }

        let mut count = 0;
        for pidx in start_pidx..self.pattern.len() {
            if Self::can_fit(&self.pattern[pidx..], self.groups[gidx]) {
                if gidx < self.groups.len() - 1 {
                    count += self.count_arrangements_rec(
                        pidx + self.groups[gidx] + 1,
                        gidx + 1,
                        last_damaged,
                        mem,
                    )
                } else if (pidx + self.groups[gidx]) >= last_damaged {
                    count += 1;
                }
            }

            if matches!(self.pattern[pidx], Spring::Damaged) {
                break;
            }
        }

        mem.insert((start_pidx, gidx), count);
        count
    }

    // can we fit 'count' damaged springs inside the pattern, from the start
    fn can_fit(pattern: &[Spring], count: usize) -> bool {
        if count > pattern.len() {
            return false;
        }

        let mut iter = pattern.iter();
        if iter
            .by_ref()
            .take(count)
            .any(|spring| matches!(spring, Spring::Operational))
        {
            return false;
        }

        !matches!(iter.next(), Some(Spring::Damaged))
    }
}

impl FromStr for Record {
    type Err = SpringErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pattern, groups) = s.trim().split_once(' ').ok_or(SpringErr::BadFormat)?;

        let pattern = pattern
            .trim()
            .chars()
            .map(|ch| match ch {
                OPERATIONAL => Ok(Spring::Operational),
                DAMAGED => Ok(Spring::Damaged),
                UNKNOWN => Ok(Spring::Unknown),
                _ => Err(SpringErr::UnknownStringCodition(ch)),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let groups = groups
            .trim()
            .split(',')
            .map(|num| {
                num.parse::<usize>()
                    .map_err(|_| SpringErr::UnknownSizeFormat)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { pattern, groups })
    }
}

#[cfg(test)]
mod tests {
    use super::{Record, Spring};

    #[test]
    fn parse_record() {
        let input = r#"???.### 1,1,3"#;
        let expected_output = Record {
            pattern: vec![
                Spring::Unknown,
                Spring::Unknown,
                Spring::Unknown,
                Spring::Operational,
                Spring::Damaged,
                Spring::Damaged,
                Spring::Damaged,
            ],
            groups: vec![1, 1, 3],
        };

        let output: Record = input.parse().unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn count_arrangements() {
        let inputs = [
            "???.### 1,1,3",
            ".??..??...?##. 1,1,3",
            "?#?#?#?#?#?#?#? 1,3,1,6",
            "????.#...#... 4,1,1",
            "????.######..#####. 1,6,5",
            "?###???????? 3,2,1",
        ];
        let expected_outputs = [1, 4, 1, 1, 4, 10];

        for (input, expected) in inputs.into_iter().zip(expected_outputs) {
            let input: Record = input.parse().unwrap();

            assert_eq!(input.count_arrangements(), expected);
        }
    }
}
