use crate::coordinate::Coordinate;
use crate::get_adjacent::distance;

const T: usize = 3; // 頂点間の距離がこれ以下だった場合は問答無用で結合
const L: usize = 50; // 頂点の最大数

// p1とp2の隣接行列から中点の隣接行列を良い感じに算出する
// 頂点配列からp1とp2を消し，中点を追加する
// 隣接行列のp1とp2に相当する部分を消し，中点に相当する部分を追加する

struct Adjacent {
    p: usize,
    q: usize,
    cost: usize,
}

pub fn merge_points(
    points: Vec<Coordinate>,
    adjacent_matrix: Vec<Vec<usize>>,
) -> (Vec<Coordinate>, Vec<Vec<usize>>) {
    let mut merged_points = points;
    let mut merged_adjacent_matrix = adjacent_matrix;

    let mut points_count = merged_points.len();

    for i in 0..points_count {
        for j in i..points_count {
            if merged_adjacent_matrix[i][j] > 0 && merged_adjacent_matrix[i][j] <= T {
                println!(
                    "({}, {})",
                    merged_adjacent_matrix.len(),
                    merged_adjacent_matrix[0].len()
                );
                (merged_points, merged_adjacent_matrix) =
                    merge_two_points(i, j, merged_points, merged_adjacent_matrix);
                points_count -= 1;
                print!("{} ", points_count);
            }
        }
    }
    println!("Merged Points");
    (merged_points, merged_adjacent_matrix)
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

    let q_max = new_points.len() - 1;

    for i in 0..q_max {
        if mid_adjacent_matrix_line[i] > 0 {
            let distance = distance(new_points[i], mid);
            new_adjacent_matrix[q_max][i] = distance;
            new_adjacent_matrix[i][q_max] = distance;
        }
    }

    (new_points, new_adjacent_matrix)
}

fn get_adjacent_matrix_line(p1: usize, p2: usize, adjacent_matrix: &Vec<Vec<usize>>) -> Vec<usize> {
    let q1 = p1.min(p2);
    let q2 = p1.max(p2);
    let q_max = adjacent_matrix.len();

    let mut ret = Vec::<usize>::new();

    for i in 0..q1 {
        ret.push(adjacent_matrix[p1][i]);
    }
    for i in q1 + 1..q2 {
        ret.push(adjacent_matrix[p1][i]);
    }
    for i in q2 + 1..q_max {
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

    let mut ret = Vec::<Coordinate>::new();

    for i in 0..q1 {
        ret.push(points[i]);
    }
    for i in q1 + 1..q2 {
        ret.push(points[i]);
    }
    for i in q1 + 1..q_max {
        ret.push(points[i]);
    }
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

    for i in 0..q1 {
        let mut ret_line = Vec::<usize>::new();
        for j in 0..q1 {
            ret_line.push(adjacent_matrix[i][j]);
        }
        for j in q1 + 1..q2 {
            ret_line.push(adjacent_matrix[i][j]);
        }
        for j in q2 + 1..q_max {
            ret_line.push(adjacent_matrix[i][j]);
        }
        ret_line.push(0);
        ret.push(ret_line);
    }
    for i in q1 + 1..q2 {
        let mut ret_line = Vec::<usize>::new();
        for j in 0..q1 {
            ret_line.push(adjacent_matrix[i][j]);
        }
        for j in q1 + 1..q2 {
            ret_line.push(adjacent_matrix[i][j]);
        }
        for j in q2 + 1..q_max {
            ret_line.push(adjacent_matrix[i][j]);
        }
        ret_line.push(0);
        ret.push(ret_line);
    }
    for i in q2 + 1..q_max {
        let mut ret_line = Vec::<usize>::new();
        for j in 0..q1 {
            ret_line.push(adjacent_matrix[i][j]);
        }
        for j in q1 + 1..q2 {
            ret_line.push(adjacent_matrix[i][j]);
        }
        for j in q2 + 1..q_max {
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
