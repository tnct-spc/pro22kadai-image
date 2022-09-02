use crate::coordinate::Coordinate;

const D: [[isize; 2]; 8] = [
    [-1, 1],
    [0, 1],
    [1, 1],
    [1, 0],
    [1, -1],
    [0, -1],
    [-1, -1],
    [-1, 0],
];

struct PointChain {
    start: usize,
    goal: usize,
    chain_code: Vec<usize>,
}

impl PointChain {
    fn new(start: usize) -> PointChain {
        PointChain {
            start,
            goal: 0,
            chain_code: vec![0],
        }
    }
}

pub fn get_adjacent_matrix(points: &Vec<Coordinate>, img: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let points_count = points.len();

    let mut lut = vec![0; points_count];
    let mut adjacent_matrix = vec![vec![0; points_count]; points_count];

    for i in 0..points_count {
        let (goal, cost) = get_chain_code(i, points, img);
        adjacent_matrix[i][goal] = cost;
    }
    adjacent_matrix
}

fn get_chain_code(start: usize, points: &Vec<Coordinate>, img: &Vec<Vec<usize>>) -> (usize, usize) {
    let mut v_old = 0;
    let mut v_new;

    let mut cost = 0;

    let mut current_point = points[start];

    println!("Start: {}", current_point);

    let points_count = points.len();

    // 最初の進行方向を定める
    for i in 0..8 {
        if is_pixel_white(current_point, &D[i], img) {
            v_old = i;
            println!("v_old: {}", v_old);
            current_point = update_current_coordinate(current_point, &D[i]);
            break;
        }
    }
    cost += 1;

    loop {
        // 探索する向きを絞り込む
        v_new = (v_old + 6) % 8;

        println!("v_new: {}", v_new);
        // 絞り込んだ対象で検索する
        for i in 0..8 {
            if v_old.abs_diff(i) != 4 && is_pixel_white(current_point, &D[i], img) {
                v_old = i;
                current_point = update_current_coordinate(current_point, &D[i]);
                cost += 1;
                let s = search_point(current_point, points);
                if s < points_count {
                    if s == start {
                        return (start, 0);
                    } else {
                        println!("{} and {} are adjacenting.", points[start], points[s]);
                        return (s, cost);
                    }
                }
            }
            if i == 7 {
                return (start, 0);
            }
        }
    }
}

fn is_pixel_white(point: Coordinate, d: &[isize; 2], img: &Vec<Vec<usize>>) -> bool {
    let x = point.x as isize + d[0];
    let y = point.y as isize + d[1];

    img[y as usize][x as usize] == 1
}

fn update_current_coordinate(current_point: Coordinate, d: &[isize; 2]) -> Coordinate {
    let x = current_point.x as isize + d[0];
    let y = current_point.y as isize + d[1];

    Coordinate {
        x: x as usize,
        y: y as usize,
    }
}

pub fn search_point(point: Coordinate, points: &Vec<Coordinate>) -> usize {
    for (i, p) in points.iter().enumerate() {
        if *p == point {
            return i;
        }
    }
    points.len()
}

pub fn print_coordinate_vec(points: &Vec<Coordinate>) {
    for p in points {
        println!("{}", *p);
    }
}
