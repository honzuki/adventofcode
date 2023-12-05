use std::{cmp::Ordering, collections::HashMap, ops::Range, str::FromStr};

#[derive(thiserror::Error, Debug)]
pub enum ParseErr {
    #[error("unknown map format")]
    BadMapFormat,

    #[error("unknown seeds format")]
    BadSeedFormat,

    #[error("the almanac is missing the seeds section")]
    MissingSeeds,

    #[error("the almanac is missing a map from {0}")]
    MissingMap(String),

    #[error("there is an uneven amount of seeds")]
    UnevenSeedCount,
}

pub struct Almanac {
    pub seeds: Vec<u64>,
    // maps the from_field to the map that maps this field
    pub maps: HashMap<String, Map>,
}

impl FromStr for Almanac {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut maps = s.split("\n\n");

        let seeds = maps.next().ok_or(ParseErr::MissingSeeds)?;
        let seeds = parse_seeds(seeds)?;

        let maps = maps
            .map(|map| map.parse::<Map>().map(|map| (map.from.clone(), map)))
            .collect::<Result<HashMap<_, _>, _>>()?;

        Ok(Self { seeds, maps })
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Rule {
    from_range: Range<u64>,
    to_start: u64,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Map {
    from: String,
    to: String,
    rules: Vec<Rule>,
}

impl Map {
    // use this function to make sure the rules are always sorted
    // by their from_range start, and then end
    fn new(from: String, to: String, mut rules: Vec<Rule>) -> Self {
        rules.sort_by_key(|rule| (rule.from_range.start, rule.from_range.end));

        Self { from, to, rules }
    }

    /// Apply the map to a value
    pub fn map(&self, value: u64) -> u64 {
        let rule = match self.rules.binary_search_by(|rule| {
            if rule.from_range.end <= value {
                Ordering::Less
            } else if rule.from_range.start > value {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }) {
            Ok(idx) => &self.rules[idx],
            Err(_) => return value,
        };

        let distance = value - rule.from_range.start;
        rule.to_start + distance
    }

    /// Apply the map to a range of values
    pub fn map_range(&self, mut range: Range<u64>) -> Vec<Range<u64>> {
        let mut ranges = vec![];

        for rule in self.rules.iter() {
            if range.is_empty() {
                break;
            }

            if rule.from_range.end <= range.start {
                continue;
            }

            // if part of the range is uncovered, map it to itself
            if rule.from_range.start > range.start {
                let move_start = range.end.min(rule.from_range.start);
                ranges.push(range.start..move_start);
                range.start = move_start;

                if range.is_empty() {
                    break;
                }
            }

            // map the range by the current rule
            let move_start = range.end.min(rule.from_range.end);
            let to_start = rule.to_start + range.start - rule.from_range.start;
            ranges.push(to_start..(to_start + (move_start - range.start)));
            range.start = move_start;
        }

        // if we still have an uncovered range, map it to itself
        if !range.is_empty() {
            ranges.push(range);
        }

        ranges
    }

    pub fn to(&self) -> &str {
        &self.to
    }
}

/// Map a seed list into ranges
pub fn get_seed_ranges(seeds: &Vec<u64>) -> Result<Vec<Range<u64>>, ParseErr> {
    if seeds.len() % 2 != 0 {
        return Err(ParseErr::UnevenSeedCount);
    }

    let mut ranges = vec![];
    for idx in (0..seeds.len()).step_by(2) {
        ranges.push(seeds[idx]..(seeds[idx] + seeds[idx + 1]));
    }

    Ok(ranges)
}

fn parse_seeds(seeds: &str) -> Result<Vec<u64>, ParseErr> {
    let (_, seeds) = seeds.split_once("seeds:").ok_or(ParseErr::BadSeedFormat)?;

    seeds
        .trim()
        .split_ascii_whitespace()
        .map(|seed| seed.parse().map_err(|_| ParseErr::BadSeedFormat))
        .collect()
}

impl FromStr for Map {
    type Err = ParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.trim().lines();

        // extract the mapping names
        let name = lines.next().ok_or(ParseErr::BadMapFormat)?;
        let name = name
            .split_ascii_whitespace()
            .next()
            .ok_or(ParseErr::BadMapFormat)?
            .trim();
        let (from, to) = name.split_once("-to-").ok_or(ParseErr::BadMapFormat)?;

        // extract the rules
        let rules = lines
            .map(|line| {
                let parts = line
                    .trim()
                    .split_ascii_whitespace()
                    .map(|part| part.parse().map_err(|_| ParseErr::BadMapFormat))
                    .collect::<Result<Vec<_>, _>>()?;
                if parts.len() != 3 {
                    return Err(ParseErr::BadMapFormat);
                }

                Ok(Rule {
                    from_range: parts[1]..(parts[1] + parts[2]),
                    to_start: parts[0],
                })
            })
            .collect::<Result<_, _>>()?;

        Ok(Map::new(from.into(), to.into(), rules))
    }
}

#[cfg(test)]
mod tests {
    use super::{Map, Rule};

    #[test]
    fn parse_map() {
        let input = r#"light-to-temperature map:
45 77 23
81 45 19
68 64 13
"#;
        let expected_output = Map {
            from: "light".into(),
            to: "temperature".into(),
            rules: vec![
                Rule {
                    from_range: 45..64,
                    to_start: 81,
                },
                Rule {
                    from_range: 64..77,
                    to_start: 68,
                },
                Rule {
                    from_range: 77..100,
                    to_start: 45,
                },
            ],
        };
        let output: Map = input.parse().unwrap();

        assert_eq!(output, expected_output);
    }

    #[test]
    fn parse_seeds() {
        let input = r"seeds: 79 14 55 13";
        let expected_output = [79, 14, 55, 13];
        let output = super::parse_seeds(input).unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn map_values() {
        let map: Map = r#"seed-to-soil map:
50 98 2
52 50 48"#
            .parse()
            .unwrap();

        let input = super::parse_seeds("seeds: 79 14 55 13").unwrap();
        let output = input.iter().map(|&seed| map.map(seed)).collect::<Vec<_>>();
        let expected_output = [81, 14, 57, 13];
        assert_eq!(output, expected_output);
    }

    #[test]
    fn get_seed_ranges() {
        let input = r"seeds: 79 14 55 13";
        let expected_output = [79..93, 55..68];
        let output = super::parse_seeds(input).unwrap();
        let output = super::get_seed_ranges(&output).unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn map_ranges() {
        let map: Map = r#"seed-to-soil map:
50 98 2
52 50 48"#
            .parse()
            .unwrap();

        let input = super::parse_seeds("seeds: 79 30 12 60").unwrap();
        let input = super::get_seed_ranges(&input).unwrap();
        let output = input
            .iter()
            .cloned()
            .map(|seed| map.map_range(seed))
            .collect::<Vec<_>>();
        let expected_output = [vec![81..100, 50..52, 100..109], vec![12..50, 52..74]];
        assert_eq!(output, expected_output);
    }
}
