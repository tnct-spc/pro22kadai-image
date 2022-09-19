use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt::{Display, Formatter};
use std::ops::Add;

pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new() -> Self {
        Self { x: 0, y: 0 }
    }
    pub fn init(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn abs(&self) -> usize {
        let x = self.x as f64;
        let y = self.y as f64;

        (x * x + y * y).sqrt() as usize
    }
    fn abs2(&self) -> usize {
        let x = self.x;
        let y = self.y;

        x * x + y * y
    }
}

impl Add for Coordinate {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Eq for Coordinate {}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_abs = self.abs2();
        let other_abs = other.abs2();

        if self_abs > other_abs {
            Ordering::Greater
        } else if self_abs < other_abs {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self.abs2() > other.abs2() {
            self
        } else {
            other
        }
    }
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self.abs2() < other.abs2() {
            self
        } else {
            other
        }
    }
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        let self_abs = self.abs2();
        let min_abs = min.abs2();
        let max_abs = max.abs2();

        if self_abs < min_abs {
            min
        } else if self_abs > max_abs {
            max
        } else {
            self
        }
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_abs = self.abs2();
        let other_abs = other.abs2();

        if self_abs > other_abs {
            Some(Ordering::Greater)
        } else if self_abs < other_abs {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
    fn lt(&self, other: &Self) -> bool {
        self.abs2() < other.abs2()
    }
    fn le(&self, other: &Self) -> bool {
        self.abs2() <= other.abs2()
    }
    fn gt(&self, other: &Self) -> bool {
        self.abs2() > other.abs2()
    }
    fn ge(&self, other: &Self) -> bool {
        self.abs2() >= other.abs2()
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.x, self.y)
    }
}
