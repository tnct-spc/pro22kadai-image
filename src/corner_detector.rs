use crate::get_adjacent::Orthant;
use crate::get_adjacent::Vector;
use std::clone;
use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::io;
use std::ops::Add;

pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl Coordinate {
    pub fn new() -> Coordinate {
        Coordinate { x: 0, y: 0 }
    }
    pub fn init(x: usize, y: usize) -> Coordinate {
        Coordinate { x, y }
    }
    pub fn coordinate_to_vec(&self, other: &Self) -> Vector {
        let x1 = self.x as isize;
        let y1 = self.y as isize;
        let x2 = other.x as isize;
        let y2 = other.y as isize;

        let mut x;
        let mut y;

        match x2.cmp(&x1) {
            Ordering::Greater => x = 1,
            Ordering::Equal => x = 0,
            Ordering::Less => x = -1,
        }
        match y2.cmp(&y1) {
            Ordering::Greater => y = 1,
            Ordering::Equal => y = 0,
            Ordering::Less => y = -1,
        }
        Vector::init(x, y)
    }
    pub fn coordinate_to_orthant(&self, other: &Self) -> Orthant {
        let v = self.coordinate_to_vec(other);
        v.vec_to_orthant()
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        if self.x == other.x && self.y == other.y {
            true
        } else {
            false
        }
    }
}

impl Add for Coordinate {
    type Output = Coordinate;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Clone for Coordinate {
    fn clone(&self) -> Self {
        Coordinate {
            x: self.x,
            y: self.y,
        }
    }
}

const D: usize = 3;

// 0 0 0
// 1 1 0
// 0 1 0
const CORNER1: [[usize; D]; D] = [[0, 0, 0], [1, 1, 0], [0, 1, 0]];

// 0 0 0
// 1 0 0
// 0 1 0
const CORNER2: [[usize; D]; D] = [[0, 0, 0], [1, 0, 0], [0, 1, 0]];

// 0 0 0
// 1 0 0
// 1 1 0
const CORNER3: [[usize; D]; D] = [[0, 0, 0], [1, 0, 0], [1, 1, 0]];

// 1 0 1
// 0 1 0
// 0 0 0
const CORNER4: [[usize; D]; D] = [[1, 0, 1], [0, 1, 0], [0, 0, 0]];

// 1 0 0
// 0 1 1
// 0 0 0
const CORNER5: [[usize; D]; D] = [[1, 0, 0], [0, 1, 1], [0, 0, 0]];

pub fn pick_corner_point(img: &Vec<Vec<usize>>) -> Vec<Coordinate> {
    let ret = apply_filter(img, &CORNER1);

    let r = apply_filter(img, &CORNER2);
    let ret = join_vec(ret, r);

    let r = apply_filter(img, &CORNER3);
    let ret = join_vec(ret, r);

    let r = apply_filter(img, &CORNER4);
    let ret = join_vec(ret, r);

    let r = apply_filter(img, &CORNER5);
    let ret = join_vec(ret, r);

    ret
}

fn apply_filter(img: &Vec<Vec<usize>>, filter: &[[usize; D]; D]) -> Vec<Coordinate> {
    let detected_points = get_coordinates(img, filter);

    let filter = flip_matrix_horizontal(filter);
    let ret = get_coordinates(img, &filter);
    let detected_points = join_vec(detected_points, ret);

    let filter = flip_matrix_vertical(&filter);
    let ret = get_coordinates(img, &filter);
    let detected_points = join_vec(detected_points, ret);

    let filter = flip_matrix_horizontal(&filter);
    let ret = get_coordinates(img, &filter);
    let detected_points = join_vec(detected_points, ret);

    detected_points
}

fn get_coordinates(img: &Vec<Vec<usize>>, filter: &[[usize; D]; D]) -> Vec<Coordinate> {
    let mut ret = Vec::new();
    let s = D / 2;

    let y_max = img.len();
    let x_max = img[0].len();

    for y in s..y_max - s {
        for x in s..x_max - s {
            if is_match_filter(img, filter, x, y) {
                ret.push(Coordinate::init(x, y));
            }
        }
    }
    ret
}

fn is_match_filter(img: &Vec<Vec<usize>>, filter: &[[usize; D]; D], x: usize, y: usize) -> bool {
    let mut ret = true;
    let s = D / 2;

    for j in 0..D {
        for i in 0..D {
            ret &= img[y - s + j][x - s + i] == filter[j][i];
        }
    }
    ret
}

fn join_vec<T>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut ret = Vec::<T>::new();

    for d in a {
        ret.push(d);
    }
    for d in b {
        ret.push(d);
    }
    ret
}

fn flip_matrix_horizontal(filter: &[[usize; D]; D]) -> [[usize; D]; D] {
    let mut ret = [[0; D]; D];

    for y in 0..D {
        for x in 0..D {
            ret[y][x] = filter[(D - 1) - y][x];
        }
    }
    ret
}

fn flip_matrix_vertical(filter: &[[usize; D]; D]) -> [[usize; D]; D] {
    let mut ret = [[0; D]; D];

    for y in 0..D {
        for x in 0..D {
            ret[y][x] = filter[y][(D - 1) - x];
        }
    }
    ret
}

fn flip_matrix(filter: &[[usize; D]; D]) -> [[usize; D]; D] {
    let mut ret = [[0; D]; D];

    for y in 0..D {
        for x in 0..D {
            ret[y][x] = filter[(D - 1) - y][(D - 1) - x];
        }
    }
    ret
}

fn match_filter(img: &Vec<Vec<usize>>, filter: &[[usize; D]; D], x: usize, y: usize) -> bool {
    let mut ret = true;

    for j in 0..D {
        for i in 0..D {
            ret &= img[y - j][x - i] == filter[j][i];
        }
    }
    ret
}

fn print_filter(filter: &[[usize; D]; D]) {
    for y in 0..D {
        print!("[");
        for x in 0..D {
            print!("{:2}", filter[y][x]);
        }
        println!("]");
    }
    println!();
}

pub fn print_coordinates(points: &Vec<Coordinate>) {
    for (i, p) in points.iter().enumerate() {
        print!("({:4}, {:4})  ", (*p).x, (*p).y);
        if i % 10 == 9 {
            println!();
        }
    }
    println!();
}
