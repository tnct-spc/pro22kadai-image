use crate::coordinate::Coordinate;
use std::clone::Clone;
use std::marker::Copy;

impl Coordinate {
    fn beside(&self, direction: Direction) -> Coordinate {
        let x = self.x as isize;
        let y = self.y as isize;

        let x = x + direction.x;
        let y = y + direction.y;

        Coordinate {
            x: x as usize,
            y: y as usize,
        }
    }
}

struct Direction {
    x: isize,
    y: isize,
}

impl Direction {
    fn new(x: isize, y: isize) -> Direction {
        Direction { x, y }
    }
}

impl Copy for Direction {}

impl Clone for Direction {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

// 2 3 4
// 1 x 5
// 0 7 6
// とりあえず真ん中は100としている
const D: [Direction; 8] = [
    Direction { x: -1, y: 1 },
    Direction { x: -1, y: 0 },
    Direction { x: -1, y: -1 },
    Direction { x: 0, y: -1 },
    Direction { x: 1, y: -1 },
    Direction { x: 1, y: 0 },
    Direction { x: 1, y: 1 },
    Direction { x: 0, y: 1 },
];

pub fn get_adjacent_matrix(points: &Vec<Coordinate>, img: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    vec![vec![0]]
}

pub fn get_beside_pixels(img: &Vec<Vec<usize>>, points: &Vec<Coordinate>) -> (usize, usize) {
    let mut target = points[0];
    let mut old_direction = 100;
    let mut cost = 0;
    let mut target_index;

    (target, old_direction) = point_next(target, old_direction, img);
    target_index = search_point(target, points);

    while target_index < 0 {
        (target, old_direction) = point_next(target, old_direction, img);
        target_index = search_point(target, points);
        println!("target_index: {}", target_index);
        cost += 1;
    }
    (target_index as usize, cost)
}

fn point_next(current: Coordinate, old_d: usize, img: &Vec<Vec<usize>>) -> (Coordinate, usize) {
    for i in 0..8 {
        if old_d.abs_diff(i) != 4 {
            if is_pixel_white(current.beside(D[old_d]), img) {
                return (current.beside(D[old_d]), i);
            }
        }
    }
    (Coordinate::new(), 100)
}

fn is_pixel_white(current: Coordinate, img: &Vec<Vec<usize>>) -> bool {
    img[current.y][current.x] > 0
}

fn search_point(target: Coordinate, points: &Vec<Coordinate>) -> isize {
    for (i, p) in points.iter().enumerate() {
        if *p == target {
            return i as isize;
        }
    }
    -1
}
