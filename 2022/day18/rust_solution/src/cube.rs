use std::{error::Error, fmt, num::ParseIntError, str::FromStr};

#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl Cube {
    pub fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }

    // returns a list of connected cubes
    pub fn connected(&self) -> Vec<Self> {
        vec![
            Self::new(self.x - 1, self.y, self.z),
            Self::new(self.x + 1, self.y, self.z),
            Self::new(self.x, self.y - 1, self.z),
            Self::new(self.x, self.y + 1, self.z),
            Self::new(self.x, self.y, self.z - 1),
            Self::new(self.x, self.y, self.z + 1),
        ]
    }

    pub fn max(&self, other: &Self) -> Self {
        Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    pub fn in_bounds(&self, min: &Self, max: &Self) -> bool {
        !(self.x < min.x
            || self.y < min.y
            || self.z < min.z
            || self.x > max.x
            || self.y > max.y
            || self.z > max.z)
    }
}

#[derive(Debug)]
pub enum CubeError {
    ValueError(ParseIntError),
    MissingValues,
}

impl Error for CubeError {}

impl fmt::Display for CubeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::MissingValues => write!(f, "missing values"),
            Self::ValueError(err) => write!(f, "{}", err),
        }
    }
}

impl FromStr for Cube {
    type Err = CubeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s
            .split(',')
            .map(|part| part.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| CubeError::ValueError(err))?;

        if coords.len() < 3 {
            return Err(CubeError::MissingValues);
        }

        Ok(Cube {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        })
    }
}
