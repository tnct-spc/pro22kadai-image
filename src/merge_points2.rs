use crate::coordinate::Coordinate;
use crate::get_adjacent::distance;

const T: usize = 3; // 頂点間のマンハッタン距離がこれ以下だった場合は問答無用で結合

// 2つの頂点を結合して隣接行列を再生成する
fn regenerate_adjacent_matrix(
    p1: usize,
    p2: usize,
    points: Vec<Coordinate>,
    adjacent_matrix: Vec<Vec<usize>>,
) -> (Vec<Coordinate>, Vec<Vec<usize>>) {
    let q1 = p1.min(p2);
    let q2 = p1.max(p2);

    // p1とp2の中点を計算する
    let mid = points[p1].mid(points[p2]);

    // 中点の隣接行列を良い感じに算出する
    let p1_adjacent_matrix_line = get_adjacent_matrix_line(p1, p2, &adjacent_matrix);
    let p2_adjacent_matrix_line = get_adjacent_matrix_line(p2, p1, &adjacent_matrix);
}

fn get_adjacent_matrix_line(p1: usize, p2: usize, adjacent_matrix: &Vec<Vec<usize>>) -> Vec<usize> {
    let q1 = p1.min(p2);
    let q2 = p1.max(p2);

    let mut ret = Vec::<usize>::new();

    for i in 0..q1 {
        ret.push(adjacent_matrix[p1][i]);
    }
    for i in q1 + 1..q2 {
        ret.push(adjacent_matrix[p1][i]);
    }
    for i in q2 + 1..adjacent_matrix.len() {
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

fn regenerate_adjacent_matrix_line(p1: usize, p2: usize, adjacent_matrix: &Vec<Vec<usize>>) {
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
    for i in 0..q_max - 2 {
        ret_line.push(0);
    }
    ret.push(ret_line);
}
