use crate::corner_detector::Coordinate;

use std::cmp::PartialEq;

struct PointChain {
    start: Coordinate,
    goal: Coordinate,
    chain: Vec<Direction>,
}

pub enum Direction {
    LeftDown,
    Down,
    RightDown,
    Right,
    RightUp,
    Up,
    LeftUp,
    Left,
}

impl Direction {
    fn from(index: usize) -> Direction {
        match index {
            0 => Direction::LeftDown,
            1 => Direction::Down,
            2 => Direction::RightDown,
            3 => Direction::Right,
            4 => Direction::RightUp,
            5 => Direction::Up,
            6 => Direction::LeftUp,
            7 => Direction::Left,
        }
    }
    fn direction_to_coordinate(&self, point: Coordinate) {}
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::LeftDown, Self::LeftDown)
            | (Self::Down, Self::Down)
            | (Self::RightDown, Self::RightDown)
            | (Self::Right, Self::Right)
            | (Self::RightUp, Self::RightUp)
            | (Self::Up, Self::Up)
            | (Self::LeftUp, Self::LeftUp)
            | (Self::Left, Self::Left) => true,
            _ => false,
        }
    }
}

// 輪郭追跡
fn get_chain_code(img: &Vec<Vec<usize>>, points: &Vec<Coordinate>) {
    // Find white pixel
    let y_max = img.len();
    let x_max = img[0].len();

    let points_count = points.len();
    let lut = vec![0; points_count];

    let mut y = 0;
    let mut x = 0;

    let mut ret = Vec::<PointChain>::new();
    for (i, p) in points.iter().enumerate() {
        let start = *p;
        for d in 0..8 {}
    }
}
