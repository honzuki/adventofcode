use std::ops::{Add, AddAssign};

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
    pub const fn into_step(self, size: isize) -> Position {
        match self {
            Self::Down => Position::new(0, size),
            Self::Left => Position::new(-size, 0),
            Self::Right => Position::new(size, 0),
            Self::Up => Position::new(0, -size),
        }
    }

    pub fn from_digit(value: char) -> Option<Self> {
        let dir = match value.to_digit(10)? {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => return None,
        };

        Some(dir)
    }
}
