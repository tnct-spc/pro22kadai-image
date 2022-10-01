use crate::adjacent_points::AdjacentPoints;
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

pub fn get_adjacent_matrix(points: &Vec<Coordinate>, img: &Vec<Vec<usize>>) -> Vec<AdjacentPoints> {
    let points_count = points.len();

    let mut ret = Vec::<AdjacentPoints>::new();

    println!("{} Points are found", points_count);

    let mut past_directions = vec![100; points_count];

    for i in 0..points_count {
        let (target, distance, direction) = get_beside_pixels(i, past_directions[i], img, points);
        past_directions[target] = direction;

        ret.push(AdjacentPoints::new(i, target, distance));
    }
    println!("Finished to get adjacent matrix");
    ret
}

pub fn get_beside_pixels(
    target: usize,
    direction: usize,
    img: &Vec<Vec<usize>>,
    points: &Vec<Coordinate>,
) -> (usize, usize, usize) {
    let mut target_point = points[target];
    let mut direction = direction;
    let mut target_index;

    (target_point, direction) = point_next(target_point, direction, img);
    target_index = search_point(target_point, points);

    while target_index < 0 {
        (target_point, direction) = point_next(target_point, direction, img);
        if direction == 100 {
            return (target, 0, direction);
        }
        target_index = search_point(target_point, points);
    }
    let distance = manhattan_distance(points[target], points[target_index as usize]);
    (target_index as usize, distance, direction)
}

pub fn euclid_distance(a: Coordinate, b: Coordinate) -> usize {
    let x = a.x.abs_diff(b.x) as f64;
    let y = a.y.abs_diff(b.y) as f64;

    (x * x + y * y).sqrt() as usize
}

pub fn manhattan_distance(a: Coordinate, b: Coordinate) -> usize {
    let x = a.x.abs_diff(b.x);
    let y = a.y.abs_diff(b.y);
    x + y
}

fn point_next(
    current: Coordinate,
    old_direction: usize,
    img: &Vec<Vec<usize>>,
) -> (Coordinate, usize) {
    for i in 0..4 {
        let j = i * 2 + 1;
        let d = old_direction.abs_diff(j);
        if old_direction == 100 || d != 4 {
            if is_pixel_white(current.beside(D[j]), img) {
                return (current.beside(D[j]), j);
            }
        }
    }
    for i in 0..4 {
        let j = i * 2;
        let d = old_direction.abs_diff(j);
        if old_direction == 100 || d != 4 {
            if is_pixel_white(current.beside(D[j]), img) {
                return (current.beside(D[j]), j);
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
