use crate::corner_detector::Coordinate;

use std::cmp::PartialEq;

pub struct Vector {
    x: isize,
    y: isize,
}

pub enum Orthant {
    Up,
    LeftUp,
    Left,
    LeftDown,
    Down,
    RightDown,
    Right,
    RightUp,
    Zero,
}

const ORTH_VECS: [Vector; 8] = [
    Vector { x: 0, y: 1 },   // Up
    Vector { x: 1, y: 1 },   // LeftUp
    Vector { x: 1, y: 0 },   // Left
    Vector { x: 1, y: -1 },  // LeftDown
    Vector { x: 0, y: -1 },  // Down
    Vector { x: -1, y: -1 }, // RightDown
    Vector { x: -1, y: 0 },  // Right
    Vector { x: -1, y: 1 },  // RightUp
];

impl Vector {
    pub fn new() -> Vector {
        Vector { x: 0, y: 0 }
    }
    pub fn init(x: isize, y: isize) -> Vector {
        Vector { x, y }
    }
    pub fn vec_to_orthant(&self) -> Orthant {
        match self.x {
            1 => match self.y {
                // Left
                1 => Orthant::LeftUp,
                0 => Orthant::Left,
                -1 => Orthant::LeftDown,
                _ => Orthant::Zero,
            },
            0 => match self.y {
                // Center
                1 => Orthant::Up,
                0 => Orthant::Zero,
                -1 => Orthant::Down,
                _ => Orthant::Zero,
            },
            -1 => match self.y {
                // Right
                1 => Orthant::RightUp,
                0 => Orthant::Right,
                -1 => Orthant::RightDown,
                _ => Orthant::Zero,
            },
            _ => Orthant::Zero,
        }
    }
}

impl PartialEq for Orthant {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Up, Self::Up)
            | (Self::LeftUp, Self::LeftUp)
            | (Self::Left, Self::Left)
            | (Self::LeftDown, Self::LeftDown)
            | (Self::Down, Self::Down)
            | (Self::RightDown, Self::RightDown)
            | (Self::Right, Self::Right)
            | (Self::RightUp, Self::RightUp)
            | (Self::Zero, Self::Zero) => true,
            _ => false,
        }
    }
}

impl Orthant {
    fn orthant_to_vec(&self) -> Vector {
        match self {
            Orthant::Up => Vector::init(0, 1),
            Orthant::LeftUp => Vector::init(1, 1),
            Orthant::Left => Vector::init(1, 0),
            Orthant::LeftDown => Vector::init(1, -1),
            Orthant::Down => Vector::init(0, -1),
            Orthant::RightDown => Vector::init(-1, 0),
            Orthant::Right => Vector::init(-1, 0),
            Orthant::RightUp => Vector::init(-1, 1),
            _ => Vector::init(0, 0),
        }
    }
    fn reverse_orthant(&self) -> Orthant {
        match self {
            Orthant::Up => Orthant::Down,
            Orthant::LeftUp => Orthant::RightDown,
            Orthant::Left => Orthant::Right,
            Orthant::LeftDown => Orthant::RightUp,
            Orthant::Down => Orthant::Up,
            Orthant::RightDown => Orthant::LeftUp,
            Orthant::Right => Orthant::Left,
            Orthant::RightUp => Orthant::LeftDown,
            _ => Orthant::Zero,
        }
    }
}

struct PointOnLine {
    coord: Coordinate,  // 現在位置
    direction: Orthant, // さっき進んだ方向
}

impl PointOnLine {
    fn new() -> PointOnLine {
        PointOnLine {
            coord: Coordinate::new(),
            direction: Orthant::Zero,
        }
    }
    fn init(coord: Coordinate, direction: Orthant) -> PointOnLine {
        PointOnLine { coord, direction }
    }
    fn next(&mut self, img: &Vec<Vec<usize>>) {
        for o in ORTH_VECS {
            if o.vec_to_orthant() != self.direction.reverse_orthant() {
                let x = (self.coord.x as isize + o.x) as usize;
                let y = (self.coord.y as isize + o.y) as usize;
                if img[y][x] == 1 {
                    self.coord.x = x;
                    self.coord.y = y;
                }
            }
        }
    }
}

pub fn get_adjacent_matrix(img: &Vec<Vec<usize>>, points: &Vec<Coordinate>) -> Vec<Vec<usize>> {
    let s = points.len();

    let mut ret = vec![vec![0; s]; s];

    for j in 0..s {
        for i in j..s {
            let s = is_adjacent(img, points[i].clone(), points[j].clone());

            ret[i][j] = s;
            ret[j][i] = s;
        }
    }
    ret
}

// 点どうしが辺でつながっているかどうかを調べる．つながっていた場合はコストを返す，つながっていなかった場合は0を返す
fn is_adjacent(img: &Vec<Vec<usize>>, start: Coordinate, goal: Coordinate) -> usize {
    let init_orth = start.coordinate_to_orthant(&goal).reverse_orthant();

    let mut current = PointOnLine::init(start, init_orth);
    0
}
