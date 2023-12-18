use std::ops::{Add, AddAssign};

pub const STEPS: [Position; 4] = [
    Direction::Down.into_step(),
    Direction::Left.into_step(),
    Direction::Right.into_step(),
    Direction::Up.into_step(),
];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub const fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for Position {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.add_assign(rhs);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub const fn into_step(self) -> Position {
        match self {
            Self::Down => Position::new(0, 1),
            Self::Left => Position::new(-1, 0),
            Self::Right => Position::new(1, 0),
            Self::Up => Position::new(0, -1),
        }
    }
}
