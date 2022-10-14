use crate::Coordinate;

const D: usize = 3;

struct NoizeFilter {
    before: [[usize; D]; D],
    after: [[usize; D]; D],
}

impl NoizeFilter {
    fn init(before: [[usize; D]; D], after: [[usize; D]; D]) -> NoizeFilter {
        NoizeFilter { before, after }
    }
    fn flip_horizontal(&self) -> Self {
        let before = flip_filter_horizontal(&self.before);
        let after = flip_filter_horizontal(&self.after);

        NoizeFilter { before, after }
    }
    fn flip_vertical(&self) -> Self {
        let before = flip_filter_vertical(&self.before);
        let after = flip_filter_vertical(&self.after);

        NoizeFilter { before, after }
    }
}

// 0 0 0
// 1 1 0
// 0 1 0
const CORNER1: [[usize; D]; D] = [[0, 0, 0], [1, 1, 0], [0, 1, 0]];

// 1 0 1
// 0 1 0
// 0 0 0
const CORNER2: [[usize; D]; D] = [[1, 0, 1], [0, 1, 0], [0, 0, 0]];

// 1 0 0
// 0 1 0
// 1 0 0
const CORNER3: [[usize; D]; D] = [[1, 0, 1], [0, 1, 0], [0, 0, 0]];

// 1 0 0
// 0 1 1
// 0 0 0
const CORNER4: [[usize; D]; D] = [[1, 0, 0], [0, 1, 1], [0, 0, 0]];

// 0 1 0    0 0 0
// 1 1 1 => 1 1 1
// 0 0 0    0 0 0
const FILTER1: NoizeFilter = NoizeFilter {
    before: [[0, 1, 0], [1, 1, 1], [0, 0, 0]],
    after: [[0, 0, 0], [1, 1, 1], [0, 0, 0]],
};

// 0 0 0    0 0 0
// 0 1 0 => 0 0 0
// 0 0 0    0 0 0
const FILTER2: NoizeFilter = NoizeFilter {
    before: [[0, 0, 0], [0, 1, 0], [0, 0, 0]],
    after: [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
};

// 0 0 0    0 0 0
// 0 1 0 => 0 0 0
// 0 1 0    0 0 0
const FILTER3: NoizeFilter = NoizeFilter {
    before: [[0, 0, 0], [0, 1, 0], [0, 1, 0]],
    after: [[0, 0, 0], [0, 0, 0], [0, 0, 0]],
};

pub fn pick_corner_point(img: &Vec<Vec<usize>>) -> Vec<Coordinate> {
    let mut ret = apply_corner_filter(img, &CORNER1);

    let r = apply_corner_filter_v(img, &CORNER2);
    ret = join_vec(ret, r);

    let r = apply_corner_filter_h(img, &CORNER3);
    ret = join_vec(ret, r);

    let r = apply_corner_filter(img, &CORNER4);
    ret = join_vec(ret, r);

    ret.sort();
    ret.dedup();

    ret
}

pub fn noize_erase(img: &mut Vec<Vec<usize>>) {
    apply_noize_filter(img, &FILTER1);
    apply_noize_filter(img, &FILTER2);
    apply_noize_filter(img, &FILTER3);
}

fn apply_corner_filter(img: &Vec<Vec<usize>>, filter: &[[usize; D]; D]) -> Vec<Coordinate> {
    let detected_points = get_coordinates(img, filter);

    let filter = flip_filter_horizontal(filter);
    let ret = get_coordinates(img, &filter);
    let detected_points = join_vec(detected_points, ret);

    let filter = flip_filter_vertical(&filter);
    let ret = get_coordinates(img, &filter);
    let detected_points = join_vec(detected_points, ret);

    let filter = flip_filter_horizontal(&filter);
    let ret = get_coordinates(img, &filter);
    let detected_points = join_vec(detected_points, ret);

    detected_points
}
fn apply_corner_filter_h(img: &Vec<Vec<usize>>, filter: &[[usize; D]; D]) -> Vec<Coordinate> {
    let detected_points = get_coordinates(img, filter);

    let filter = flip_filter_horizontal(filter);
    let ret = get_coordinates(img, &filter);
    let detected_points = join_vec(detected_points, ret);

    detected_points
}

fn apply_corner_filter_v(img: &Vec<Vec<usize>>, filter: &[[usize; D]; D]) -> Vec<Coordinate> {
    let detected_points = get_coordinates(img, filter);

    let filter = flip_filter_vertical(filter);
    let ret = get_coordinates(img, &filter);
    let detected_points = join_vec(detected_points, ret);

    detected_points
}

fn apply_noize_filter(img: &mut Vec<Vec<usize>>, filter: &NoizeFilter) {
    replace_filter(img, &filter);

    let filter = filter.flip_horizontal();
    replace_filter(img, &filter);

    let filter = filter.flip_vertical();
    replace_filter(img, &filter);

    let filter = filter.flip_horizontal();
    replace_filter(img, &filter);
}

fn get_coordinates(img: &Vec<Vec<usize>>, filter: &[[usize; D]; D]) -> Vec<Coordinate> {
    let mut ret = Vec::new();
    let s = D / 2;

    let y_max = img.len();
    let x_max = img[0].len();

    for y in s..y_max - s {
        for x in s..x_max - s {
            if match_filter(img, filter, Coordinate::init(x, y)) {
                ret.push(Coordinate::init(x, y));
            }
        }
    }
    ret
}

fn match_filter(img: &Vec<Vec<usize>>, filter: &[[usize; D]; D], point: Coordinate) -> bool {
    let mut ret = true;
    let s = D / 2;

    for j in 0..D {
        for i in 0..D {
            ret &= img[point.y - s + j][point.x - s + i] == filter[j][i];
        }
    }
    ret
}

// Replace before as after when filter matches
fn replace_filter(img: &mut Vec<Vec<usize>>, filter: &NoizeFilter) {
    let y_max = img.len();
    let x_max = img[0].len();

    let s = D / 2;

    for y in 1..y_max - 1 {
        for x in 1..x_max - 1 {
            if match_filter(img, &filter.before, Coordinate::init(x, y)) {
                for j in 0..D {
                    for i in 0..D {
                        img[y - s + j][x - s + i] = filter.after[j][i];
                    }
                }
            }
        }
    }
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

fn flip_filter_horizontal(filter: &[[usize; D]; D]) -> [[usize; D]; D] {
    let mut ret = [[0; D]; D];

    for y in 0..D {
        for x in 0..D {
            ret[y][x] = filter[(D - 1) - y][x];
        }
    }
    ret
}

fn flip_filter_vertical(filter: &[[usize; D]; D]) -> [[usize; D]; D] {
    let mut ret = [[0; D]; D];

    for y in 0..D {
        for x in 0..D {
            ret[y][x] = filter[y][(D - 1) - x];
        }
    }
    ret
}

fn flip_filter(filter: &[[usize; D]; D]) -> [[usize; D]; D] {
    let mut ret = [[0; D]; D];

    for y in 0..D {
        for x in 0..D {
            ret[y][x] = filter[(D - 1) - y][(D - 1) - x];
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
