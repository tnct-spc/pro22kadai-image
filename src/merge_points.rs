use std::clone::Clone;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::marker::Copy;

use crate::coordinate::Coordinate;
use crate::get_adjacent::distance;

const T: usize = 1; // 頂点間の距離がこれ以下だった場合は問答無用で結合
const UPPER_LIMIT: usize = 50; // 頂点の最大数
const LOWER_LIMIT: usize = 3; // 頂点の最小数（これ以下だとマッチングでエラーを吐く）

// p1とp2の隣接行列から中点の隣接行列を良い感じに算出する
// 頂点配列からp1とp2を消し，中点を追加する
// 隣接行列のp1とp2に相当する部分を消し，中点に相当する部分を追加する

struct Adjacent {
    p: usize,
    q: usize,
    cost: usize,
}

impl Adjacent {
    fn init(p: usize, q: usize, cost: usize) -> Self {
        Adjacent { p, q, cost }
    }
}

impl Copy for Adjacent {}

impl Clone for Adjacent {
    fn clone(&self) -> Self {
        Adjacent {
            p: self.p,
            q: self.q,
            cost: self.cost,
        }
    }
}

// costでソートするために比較演算子をオーバーライドする
impl Eq for Adjacent {}

impl PartialEq for Adjacent {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
    fn ne(&self, other: &Self) -> bool {
        self.cost != other.cost
    }
}

impl Ord for Adjacent {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cost > other.cost {
            Ordering::Greater
        } else if self.cost < other.cost {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self.cost > other.cost {
            self
        } else {
            other
        }
    }
    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        if self.cost < other.cost {
            self
        } else {
            other
        }
    }
    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized,
    {
        if self.cost > max.cost {
            max
        } else if self.cost < min.cost {
            min
        } else {
            self
        }
    }
}

impl PartialOrd for Adjacent {
    fn lt(&self, other: &Self) -> bool {
        self.cost < other.cost
    }
    fn le(&self, other: &Self) -> bool {
        self.cost <= other.cost
    }
    fn gt(&self, other: &Self) -> bool {
        self.cost > other.cost
    }
    fn ge(&self, other: &Self) -> bool {
        self.cost >= other.cost
    }
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.cost > other.cost {
            return Some(Ordering::Greater);
        }
        if self.cost < other.cost {
            return Some(Ordering::Less);
        }
        if self.cost == other.cost {
            return Some(Ordering::Equal);
        }
        None
    }
}

pub fn merge_points(
    points: Vec<Coordinate>,
    adjacent_matrix: Vec<Vec<usize>>,
) -> (Vec<Coordinate>, Vec<Vec<usize>>) {
    let mut points = points;
    let mut adjacent_matrix = adjacent_matrix;

    let mut adjacents = generate_adjacents(&points, &adjacent_matrix);
    adjacents.sort();

    // 頂点間の距離がしきい値以下のものを結合する
    // while adjacents[0].cost <= T {
    //     if points.len() <= LOWER_LIMIT {
    //         return (points, adjacent_matrix);
    //     }
    //     // let a = adjacents.pop().unwrap();
    //     let a=adjacents[0];
    //     (points, adjacent_matrix) = merge_two_points(a.p, a.q, points, adjacent_matrix);
    //     adjacents = generate_adjacents(&points, &adjacent_matrix);
    //     adjacents.sort();
    // }
    // 頂点の数がしきい値以下になるまで頂点を距離が近い順に結合する
    adjacents.sort();
    let mut points_count = points.len();
    while points_count > UPPER_LIMIT {
        if points.len() <= LOWER_LIMIT {
            return (points, adjacent_matrix);
        }
        // let a = adjacents.pop().unwrap();
        let a = adjacents[0];
        (points, adjacent_matrix) = merge_two_points(a.p, a.q, points, adjacent_matrix);
        adjacents = generate_adjacents(&points, &adjacent_matrix);
        adjacents.sort();
        points_count = points.len();
    }
    // println!("Merged Points");
    (points, adjacent_matrix)
}

// 隣接行列をもとに頂点の接続情報を生成する
fn generate_adjacents(
    points: &Vec<Coordinate>,
    adjacent_matrix: &Vec<Vec<usize>>,
) -> Vec<Adjacent> {
    let points_count = points.len();
    let mut ret = Vec::new();

    for i in 0..points_count {
        for j in i..points_count {
            if adjacent_matrix[i][j] > 0 {
                ret.push(Adjacent::init(i, j, adjacent_matrix[i][j]));
            }
        }
    }
    ret
}

// 2つの頂点を結合して隣接行列を再生成する
fn merge_two_points(
    p1: usize,
    p2: usize,
    points: Vec<Coordinate>,
    adjacent_matrix: Vec<Vec<usize>>,
) -> (Vec<Coordinate>, Vec<Vec<usize>>) {
    // 対象の頂点p1，p2と隣接行列，頂点配列を受け取る
    // p1とp2の中点を計算する
    let mid = points[p1].mid(points[p2]);

    // 中点の隣接行列を良い感じに算出する
    let p1_adjacent_matrix_line = get_adjacent_matrix_line(p1, p2, &adjacent_matrix);
    let p2_adjacent_matrix_line = get_adjacent_matrix_line(p2, p1, &adjacent_matrix);

    let mid_adjacent_matrix_line =
        merge_adjacent_matrix_lines(p1_adjacent_matrix_line, p2_adjacent_matrix_line);

    let new_points = regenerate_points(p1, p2, mid, points);
    let mut new_adjacent_matrix = regenerate_adjacent_matrix(p1, p2, adjacent_matrix);

    let q_max = new_points.len();

    // println!("q_max: {}", q_max);
    // println!(
    //     "adjacent_matrix len: ({}, {})",
    //     new_adjacent_matrix.len(),
    //     new_adjacent_matrix[0].len()
    // );
    for i in 0..q_max {
        if mid_adjacent_matrix_line[i] > 0 {
            let distance = distance(new_points[i], mid);
            new_adjacent_matrix[q_max - 1][i] = distance;
            new_adjacent_matrix[i][q_max - 1] = distance;
        }
    }

    (new_points, new_adjacent_matrix)
}

fn get_adjacent_matrix_line(p1: usize, p2: usize, adjacent_matrix: &Vec<Vec<usize>>) -> Vec<usize> {
    let q1 = p1.min(p2);
    let q2 = p1.max(p2);
    let q_max = adjacent_matrix.len();

    let mut ret = Vec::<usize>::new();

    for i in 0..q_max {
        if i == q1 || i == q2 {
            continue;
        }
        ret.push(adjacent_matrix[p1][i]);
    }
    ret.push(0);
    ret
}

fn merge_adjacent_matrix_lines(
    p1_adjacent_matrix_line: Vec<usize>,
    p2_adjacent_matrix_line: Vec<usize>,
) -> Vec<usize> {
    let mut ret = Vec::<usize>::new();
    for i in 0..p1_adjacent_matrix_line.len() {
        ret.push(p1_adjacent_matrix_line[i] + p2_adjacent_matrix_line[i]);
    }
    ret
}

fn regenerate_points(
    p1: usize,
    p2: usize,
    mid: Coordinate,
    points: Vec<Coordinate>,
) -> Vec<Coordinate> {
    let q1 = p1.min(p2);
    let q2 = p1.max(p2);
    let q_max = points.len();

    let mut ret = Vec::new();

    for i in 0..q_max {
        if i == q1 || i == q2 {
            continue;
        }
        ret.push(points[i]);
    }

    /*
    for i in 0..q1 {
        ret.push(points[i]);
    }
    for i in (q1 + 1)..q2 {
        ret.push(points[i]);
    }
    for i in (q1 + 1)..q_max {
        ret.push(points[i]);
    }
    */
    ret.push(mid);
    ret
}

// 隣接行列を再生成する
// 肝心のmidの行/列は0なので，この後にp1とp2をマージしたやつをもとに距離を再計算して隣接行列を再生成する
fn regenerate_adjacent_matrix(
    p1: usize,
    p2: usize,
    adjacent_matrix: Vec<Vec<usize>>,
) -> Vec<Vec<usize>> {
    let q1 = p1.min(p2);
    let q2 = p1.max(p2);
    let q_max = adjacent_matrix.len();

    let mut ret = Vec::<Vec<usize>>::new();

    for i in 0..q_max {
        let mut ret_line = Vec::new();
        if i == q1 || i == q2 {
            continue;
        }
        for j in 0..q_max {
            if j == q1 || j == q2 {
                continue;
            }
            ret_line.push(adjacent_matrix[i][j]);
        }
        ret_line.push(0);
        ret.push(ret_line);
    }
    let mut ret_line = Vec::<usize>::new();
    for _ in 0..q_max - 1 {
        ret_line.push(0);
    }
    ret.push(ret_line);
    ret
}
