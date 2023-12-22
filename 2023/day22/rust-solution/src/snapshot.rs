use std::{
    collections::{HashMap, HashSet},
    ops::Range,
    str::FromStr,
};

#[derive(thiserror::Error, Debug)]
pub enum SnapshotErr {
    #[error("unknown brick format: {0}")]
    BadBrick(String),

    #[error("failed to parse coordinate with value: {0}")]
    BadCoordinate(String),

    #[error("a position should be represented by 3 coordinates")]
    MissingCoordinates,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Brick {
    x_range: Range<usize>,
    y_range: Range<usize>,
    z_range: Range<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stack {
    bricks: Vec<Brick>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Support {
    support: HashSet<usize>,
    supported_by: HashSet<usize>,
}

impl Stack {
    fn new(mut bricks: Vec<Brick>) -> Self {
        bricks.sort_by(|a, b| a.z_range.end.cmp(&b.z_range.end));

        Self { bricks }
    }

    pub fn after_fall(mut self) -> Self {
        let mut zmap: HashMap<(usize, usize), usize> = Default::default();
        for brick in self.bricks.iter_mut() {
            let xy_map = brick
                .x_range
                .clone()
                .flat_map(|x| brick.y_range.clone().map(move |y| (x, y)));

            // find the highest supporting point the brick can fall into
            let highest_point = xy_map
                .clone()
                .map(|xy| *zmap.entry(xy).or_default())
                .max()
                .unwrap();

            debug_assert!(brick.z_range.start > highest_point);
            let fall_distance = brick.z_range.start - highest_point - 1;
            brick.z_range.start -= fall_distance;
            brick.z_range.end -= fall_distance;

            // add the brick to the zmap
            xy_map.for_each(|xy| {
                zmap.insert(xy, brick.z_range.end - 1);
            });
        }

        Self::new(self.bricks)
    }

    pub fn count_bricks_that_can_be_disintegrated(&self) -> usize {
        let supports = self.build_support_vec();

        supports
            .iter()
            .filter(|support| {
                support
                    .support
                    .iter()
                    .all(|&idx| supports[idx].supported_by.len() > 1)
            })
            .count()
    }

    pub fn count_brick_that_will_fall_after_disintegration(&self) -> usize {
        let supports = self.build_support_vec();

        (0..self.bricks.len())
            .map(|idx| {
                self.count_brick_that_will_fall_after_disintegration_of_specific_block(
                    supports.clone(),
                    idx,
                )
            })
            .sum()
    }

    fn count_brick_that_will_fall_after_disintegration_of_specific_block(
        &self,
        mut supports: Vec<Support>,
        block_idx: usize,
    ) -> usize {
        let mut frontier = supports[block_idx]
            .support
            .iter()
            .filter(|&&idx| supports[idx].supported_by.len() == 1)
            .cloned()
            .collect::<Vec<_>>();

        let mut fall = 0;
        while let Some(idx) = frontier.pop() {
            for sidx in supports[idx].support.clone().into_iter() {
                supports[sidx].supported_by.remove(&idx);
                if supports[sidx].supported_by.is_empty() {
                    frontier.push(sidx);
                }
            }

            fall += 1;
        }

        fall
    }

    fn build_support_vec(&self) -> Vec<Support> {
        let mut map = HashMap::new();
        for (idx, brick) in self.bricks.iter().enumerate() {
            for x in brick.x_range.clone() {
                for y in brick.y_range.clone() {
                    for z in brick.z_range.clone() {
                        map.insert(Point::new(x, y, z), idx);
                    }
                }
            }
        }

        let mut supports = (0..self.bricks.len())
            .map(|_| Support {
                support: Default::default(),
                supported_by: Default::default(),
            })
            .collect::<Vec<_>>();
        for (idx, brick) in self.bricks.iter().enumerate() {
            for x in brick.x_range.clone() {
                for y in brick.y_range.clone() {
                    if let Some(&sidx) = map.get(&Point::new(x, y, brick.z_range.end)) {
                        supports[idx].support.insert(sidx);
                        supports[sidx].supported_by.insert(idx);
                    }
                }
            }
        }

        supports
    }
}

impl Brick {
    fn new(start: Point, end: Point) -> Self {
        Self {
            x_range: (start.x.min(end.x)..(start.x.max(end.x) + 1)),
            y_range: (start.y.min(end.y)..(start.y.max(end.y) + 1)),
            z_range: (start.z.min(end.z)..(start.z.max(end.z) + 1)),
        }
    }
}

impl FromStr for Point {
    type Err = SnapshotErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .trim()
            .split(',')
            .map(|n| {
                n.parse::<usize>()
                    .map_err(|_| SnapshotErr::BadCoordinate(n.into()))
            })
            .collect::<Result<Vec<_>, _>>()?;

        if parts.len() < 3 {
            return Err(SnapshotErr::MissingCoordinates);
        }

        Ok(Self::new(parts[0], parts[1], parts[2]))
    }
}

impl FromStr for Brick {
    type Err = SnapshotErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s
            .trim()
            .split_once('~')
            .ok_or_else(|| SnapshotErr::BadBrick(s.into()))?;

        Ok(Self::new(start.parse()?, end.parse()?))
    }
}

impl FromStr for Stack {
    type Err = SnapshotErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bricks = s
            .trim()
            .lines()
            .map(|line| line.parse::<Brick>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self::new(bricks))
    }
}
