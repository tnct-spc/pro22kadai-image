use crate::coordinate::Coordinate;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt::Display;

pub struct AdjacentPoints {
    pub start: usize,
    pub goal: usize,
    pub distance: usize,
}

impl AdjacentPoints {
    pub fn new(start: usize, goal: usize, distance: usize) -> Self {
        AdjacentPoints {
            start,
            goal,
            distance,
        }
    }
}

impl PartialEq for AdjacentPoints {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl Eq for AdjacentPoints {}

impl PartialOrd for AdjacentPoints {
    fn ge(&self, other: &Self) -> bool {
        self.distance >= other.distance
    }
    fn gt(&self, other: &Self) -> bool {
        self.distance > other.distance
    }
    fn le(&self, other: &Self) -> bool {
        self.distance <= other.distance
    }
    fn lt(&self, other: &Self) -> bool {
        self.distance < other.distance
    }
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for AdjacentPoints {
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        if self.distance < min.distance {
            min
        } else if self.distance > max.distance {
            max
        } else {
            self
        }
    }
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance > other.distance {
            Ordering::Greater
        } else if self.distance < other.distance {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self.distance > other.distance {
            self
        } else {
            other
        }
    }
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self.distance < other.distance {
            self
        } else {
            other
        }
    }
}

impl Display for AdjacentPoints {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]-{}-[{}]", self.start, self.distance, self.goal,)
    }
}
