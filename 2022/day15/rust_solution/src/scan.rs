use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
pub struct Position(isize, isize);

impl Position {
    pub fn new(x: isize, y: isize) -> Position {
        Position(x, y)
    }
}

impl Position {
    pub fn manhattan_distance(&self, other: &Self) -> usize {
        self.x_dist(other.0) + self.y_dist(other.1)
    }

    pub fn x_dist(&self, x: isize) -> usize {
        self.0.abs_diff(x)
    }

    pub fn y_dist(&self, y: isize) -> usize {
        self.1.abs_diff(y)
    }

    pub fn get_x(&self) -> isize {
        self.0
    }

    pub fn get_y(&self) -> isize {
        self.1
    }
}

#[derive(Debug)]
pub struct Scan {
    pub sensor: Position,
    pub beacon: Position,
}

impl FromStr for Scan {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re: regex::Regex =
            Regex::new(r".*?x=(-?\d+), y=(-?\d+):.*?x=(-?\d+), y=(-?\d+).*").unwrap();
        let cap = re.captures(s).unwrap();

        Ok(Scan {
            sensor: Position::new(
                cap[1].parse::<isize>().unwrap(),
                cap[2].parse::<isize>().unwrap(),
            ),
            beacon: Position::new(
                cap[3].parse::<isize>().unwrap(),
                cap[4].parse::<isize>().unwrap(),
            ),
        })
    }
}
