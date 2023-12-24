use std::{ops::Deref, str::FromStr};

#[derive(thiserror::Error, Debug)]
pub enum HailErr {
    #[error("unknown hail stone format: {0}")]
    BadHailStone(String),

    #[error("expected a list of 3 numbers")]
    BadVec3(String),

    #[error("hailstone can not have zero x velocity")]
    ZeroVel,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn from_xy(x: f64, y: f64) -> Self {
        Self { x, y, z: 0.0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Line {
    m: f64,
    b: f64,
}

impl Line {
    fn intersect(&self, other: &Self) -> Option<Vec3> {
        // y = mx + b
        // y1 = y2 => m1x + b1 = m2x + b2 => (m1 - m2)x = b2 - b1
        // => x = (b2 - b1) / (m1 - m2) [when (m1 - m2) != 0]

        let mdiff = self.m - other.m;
        if mdiff == 0.0 {
            // no solution
            return None;
        }

        let x = (other.b - self.b) / mdiff;
        let y = (self.m * x) + self.b;
        Some(Vec3::from_xy(x, y))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct HailStone {
    start: Vec3,
    vel: Vec3,
}

impl HailStone {
    fn contains_xy(&self, point: &Vec3) -> bool {
        let xdiff = point.x - self.start.x;
        let ydiff = point.y - self.start.y;

        let xvel = self.vel.x / xdiff;
        let yvel = self.vel.y / ydiff;

        (xvel - yvel) < 0.0000000001 && xvel >= 0.0
    }

    fn get_future_pos(&self, secs: f64) -> Vec3 {
        Vec3::new(
            self.start.x + (self.vel.x * secs),
            self.start.y + (self.vel.y * secs),
            self.start.z + (self.vel.z * secs),
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HailStones {
    data: Vec<HailStone>,
}

impl HailStones {
    pub fn predict_collisions(&self, min_vec: &Vec3, max_vec: &Vec3) -> usize {
        let lines: Vec<Line> = self.data.iter().map(|hs| (*hs).into()).collect::<Vec<_>>();
        self.data
            .iter()
            .enumerate()
            .map(|(idx1, hs1)| {
                ((idx1 + 1)..self.data.len())
                    .filter(|idx2| {
                        let Some(intersect) = lines[idx1].intersect(&lines[*idx2]) else {
                            return false;
                        };
                        if intersect.x < min_vec.x
                            || intersect.y < min_vec.y
                            || intersect.x > max_vec.x
                            || intersect.y > max_vec.y
                        {
                            return false;
                        }

                        hs1.contains_xy(&intersect) && self.data[*idx2].contains_xy(&intersect)
                    })
                    .count()
            })
            .sum()
    }
}

impl From<HailStone> for Line {
    fn from(value: HailStone) -> Self {
        let p1 = value.start;
        let p2 = value.get_future_pos(1.0);

        let ydiff = p2.y - p1.y;
        let xdiff = p2.x - p1.x;
        debug_assert!(xdiff != 0.0);

        let m = ydiff / xdiff;

        // y = mx + b => b = y - mx
        let b = p1.y - (m * p1.x);

        Self { m, b }
    }
}

impl FromStr for Vec3 {
    type Err = HailErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .trim()
            .split(',')
            .map(|part| {
                part.trim()
                    .parse::<f64>()
                    .map_err(|_| HailErr::BadVec3(s.into()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        if parts.len() != 3 {
            return Err(HailErr::BadVec3(s.into()));
        }

        Ok(Self::new(parts[0], parts[1], parts[2]))
    }
}

impl FromStr for HailStone {
    type Err = HailErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = s
            .trim()
            .split_once('@')
            .ok_or_else(|| HailErr::BadHailStone(s.into()))?;

        let vel: Vec3 = vel.parse()?;
        if vel.x == 0.0 {
            return Err(HailErr::ZeroVel);
        }

        Ok(Self {
            start: pos.parse()?,
            vel,
        })
    }
}

impl FromStr for HailStones {
    type Err = HailErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .trim()
            .lines()
            .map(|line| line.parse::<HailStone>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { data })
    }
}

#[cfg(test)]
mod tests {
    use super::{HailStone, HailStones, Line, Vec3};

    #[test]
    fn parse_hailstone() {
        let input = "18, 19, 22 @ -1, -1, -2";
        let expected_output = HailStone {
            start: Vec3::new(18.0, 19.0, 22.0),
            vel: Vec3::new(-1.0, -1.0, -2.0),
        };

        let output: HailStone = input.parse().unwrap();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn get_line() {
        let hs: HailStone = "19, 13, 30 @ -2, 1, -2".parse().unwrap();
        let expected_output = Line { m: -0.5, b: 22.5 };

        let output: Line = hs.into();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn check_intersect() {
        let l1: Line = "18, 19, 22 @ -1, -1, -2"
            .parse::<HailStone>()
            .unwrap()
            .into();
        let l2: Line = "12, 31, 28 @ -1, -2, -1"
            .parse::<HailStone>()
            .unwrap()
            .into();

        assert_eq!(l1.intersect(&l2).unwrap(), Vec3::from_xy(-6.0, -5.0))
    }

    #[test]
    fn check_contains_xy() {
        let hs1: HailStone = "19, 13, 30 @ -2, 1, -2".parse().unwrap();
        let hs2: HailStone = "18, 19, 22 @ -1, -1, -2".parse().unwrap();

        let l1: Line = hs1.into();
        let l2: Line = hs2.into();

        let intersect = l1.intersect(&l2).unwrap();
        assert!(hs1.contains_xy(&intersect));

        let hs: HailStone = "19, 13, 30 @ -2, 1, -2".parse().unwrap();
        assert!(!hs.contains_xy(&Vec3::from_xy(21.0, 12.0)));
    }

    #[test]
    fn check_predict_collisions() {
        let input: HailStones = r#"19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3"#
            .parse()
            .unwrap();
        let expected_output = 2;

        assert_eq!(
            input.predict_collisions(&Vec3::from_xy(7.0, 7.0), &Vec3::from_xy(27.0, 27.0)),
            expected_output
        );
    }
}
