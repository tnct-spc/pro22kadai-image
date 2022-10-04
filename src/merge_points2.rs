use crate::coordinate::Coordinate;

const T: usize = 3; // 頂点間のマンハッタン距離がこれ以下だった場合は問答無用で結合

// 2つの頂点を結合して隣接行列を再生成する
fn merge_two_points(
    p1: usize,
    p2: usize,
    points: Vec<Coordinate>,
    adjacent_matrix: Vec<Vec<usize>>,
) -> (Vec<Coordinate>, Vec<Vec<usize>>) {
    let p1 = p1.min(p2);
    let p2 = p1.max(p2);

    let p_max = points.len();
    let new_p_max = p_max - 1;

    let mid_point = points[p1].mid(points[p2]);

    let mut new_adjacent_matrix = Vec::<Vec<usize>>::new();
    let mut new_points = Vec::<Coordinate>::new();

    // 隣接行列のp1とp2の分を分離する
    let p1_adjacenting_points = get_adjacenting_points(p1, &adjacent_matrix);
    let p2_adjacenting_points = get_adjacenting_points(p2, &adjacent_matrix);
    // p1とp2の隣接頂点をマージする

    // 頂点のVecから結合するやつを消し飛ばし，中点を追加する
    for i in 0..p_max {
        if i == p1 || i == p2 {
            continue;
        }
        new_points.push(points[i]);
    }
    new_points.push(mid_point);

    (new_points, new_adjacent_matrix)
}

fn get_adjacenting_points(index: usize, adjacent_matrix: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut ret = Vec::<usize>::new();
    for i in 0..adjacent_matrix.len() {
        ret.push(adjacent_matrix[index][i]);
    }
    ret
}

fn merge_adjacenting_points(
    p1: usize,
    p2: usize,
    p1_adjacenting_matrix: Vec<usize>,
    p2_adjacenting_matrix: Vec<usize>,
) -> Vec<usize> {
    let points_count = p1_adjacenting_matrix.len();
    let lut = vec![0; points_count];

    vec![0]
}
