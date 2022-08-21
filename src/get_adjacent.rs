use crate::corner_detector::Coordinate;

enum Orthant {
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

pub fn get_adjacent_matrix(img: &Vec<Vec<usize>>, points: &Vec<Coordinate>) -> Vec<Vec<usize>> {
    let s = points.len();

    let mut ret = vec![vec![0; s]; s];

    for j in 0..s {
        for i in j..s {
            ret[i][j] = is_adjacent(img, &points[i], &points[j]);
            ret[j][i] = is_adjacent(img, &points[i], &points[j]);
        }
    }
    ret
}

// 点どうしが辺でつながっているかどうかを調べる．つながっていた場合はコストを返す，つながっていなかった場合は0を返す
fn is_adjacent(img: &Vec<Vec<usize>>, start: &Coordinate, goal: &Coordinate) -> usize {
    let mut current = start;

    let mut past_coord = Coordinate::new();

    loop {}
    0
}
