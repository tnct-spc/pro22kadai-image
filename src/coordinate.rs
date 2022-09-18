use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::cmp::Ordering::Equal;
use std::intrinsics::sqrtf64;
use std::ops::Add;

pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
    fn init(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    fn abs(&self) -> usize {
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

impl Eq for Coordinate {}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Ord for Coordinate {
    fn min(self, other: Self) -> Self where Self: Sized {
        if self.abs2() < other.abs2() {
            self
        } else {
            other
        }
    }
    fn max(self, other: Self) -> Self where Self: Sized {
        if self.abs2() > other.abs2() {
            self
        } else {
            other
        }
    }
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
    fn clamp(self, min: Self, max: Self) -> Self where Self: Sized {
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