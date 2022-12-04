use std::env;
use std::error::Error;
use std::fs;
use std::io;

fn read_input() -> io::Result<String> {
    let args: Vec<_> = env::args().collect();
    let input_path = args.get(1).expect("missing input file path");

    fs::read_to_string(input_path)
}

struct Range {
    min: i32,
    max: i32,
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.min <= other.min && self.max >= other.max
    }

    fn overlap(&self, other: &Self) -> bool {
        self.max >= other.min && self.max <= other.max
            || other.max >= self.min && other.max <= self.max
    }
}

impl std::str::FromStr for Range {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min, max) = s.split_once('-').unwrap();

        let min = min.parse::<i32>()?;
        let max = max.parse::<i32>()?;

        Ok(Range { min, max })
    }
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|range| range.parse::<Range>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|line| line[0].contains(&line[1]) | line[1].contains(&line[0]))
        .count()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|range| range.parse::<Range>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|line| line[0].overlap(&line[1]))
        .count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_input()?;
    println!("part 1 result: {}", part_1(&input));
    println!("part 2 result: {}", part_2(&input));

    Ok(())
}
