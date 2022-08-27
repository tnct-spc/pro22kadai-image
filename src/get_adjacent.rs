use crate::corner_detector::Coordinate;

use std::cmp::PartialEq;

pub struct Direction {
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

const ORTH_VECS: [Direction; 8] = [
    Direction { x: 0, y: 1 },   // Up
    Direction { x: 1, y: 1 },   // LeftUp
    Direction { x: 1, y: 0 },   // Left
    Direction { x: 1, y: -1 },  // LeftDown
    Direction { x: 0, y: -1 },  // Down
    Direction { x: -1, y: -1 }, // RightDown
    Direction { x: -1, y: 0 },  // Right
    Direction { x: -1, y: 1 },  // RightUp
];

impl Direction {
    pub fn new() -> Direction {
        Direction { x: 0, y: 0 }
    }
    pub fn init(x: isize, y: isize) -> Direction {
        Direction { x, y }
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
    fn orthant_to_vec(&self) -> Direction {
        match self {
            Orthant::Up => Direction::init(0, 1),
            Orthant::LeftUp => Direction::init(1, 1),
            Orthant::Left => Direction::init(1, 0),
            Orthant::LeftDown => Direction::init(1, -1),
            Orthant::Down => Direction::init(0, -1),
            Orthant::RightDown => Direction::init(-1, 0),
            Orthant::Right => Direction::init(-1, 0),
            Orthant::RightUp => Direction::init(-1, 1),
            _ => Direction::init(0, 0),
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
    fn next(&mut self, img: &Vec<Vec<usize>>) -> bool {
        let y_max = img.len() as isize;
        let x_max = img[0].len() as isize;

        for o in ORTH_VECS {
            if o.vec_to_orthant() != self.direction.reverse_orthant() {
                let x = self.coord.x as isize + o.x;
                let y = self.coord.y as isize + o.y;

                if x <= 0 || y <= 0 || x > x_max - 1 || y > y_max - 1 {
                    return false;
                }
                if img[y as usize][x as usize] == 1 {
                    self.coord.x = x as usize;
                    self.coord.y = y as usize;
                    return true;
                }
            }
        }
        false
    }
}

pub fn get_adjacent_matrix(img: &Vec<Vec<usize>>, points: &Vec<Coordinate>) -> Vec<Vec<usize>> {
    let s = points.len();

    let mut ret = vec![vec![0; s]; s];

    for j in 0..s {
        for i in j..s {
            println!("Target points: {}, {}", points[i], points[j]);

            let start = points[i];
            let goal = points[j];

            let init_orth = start.coordinate_to_orthant(&goal).reverse_orthant();
            let mut current = PointOnLine::init(start, init_orth);
            let mut cost: usize = 0;

            loop {
                cost += 1;
                let s = current.next(img);
                if s {
                    let p = search_points(current.coord, points);
                    if p >= 0 {
                        ret[i][p as usize] = cost;
                        ret[p as usize][i] = cost;
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    }
    return ret;
}

// 点どうしが辺でつながっているかどうかを調べる．つながっていた場合はコストを返す，つながっていなかった場合は0を返す
fn is_adjacent(
    img: &Vec<Vec<usize>>,
    points: &Vec<Coordinate>,
    start: Coordinate,
    goal: Coordinate,
) -> usize {
    let init_orth = start.coordinate_to_orthant(&goal).reverse_orthant();

    let mut current = PointOnLine::init(start, init_orth);

    let mut cost = 0;

    while current.next(img) {
        cost += 1;

        if current.coord == goal {
            println!("Two points are adjacenting");
            return cost;
        }
        if search_points(current.coord, points) == 0 {
            println!("Another points are adjacenting");
            return 0;
        }
    }
    0
}

fn search_points(target: Coordinate, points: &Vec<Coordinate>) -> isize {
    for (i, p) in points.iter().enumerate() {
        if target == *p {
            return i as isize;
        }
    }
    -1
}
