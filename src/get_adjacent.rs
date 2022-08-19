use crate::corner_detector::Coordinate;

pub fn get_adjacent_matrix(img: &Vec<Vec<usize>>, points: &Vec<Coordinate>) -> Vec<Vec<usize>> {
    let s = points.len();

    let mut ret = vec![vec![0; s]; s];

    for j in 0..s {
        for i in j..s {
            ret[i][j] = is_adjacent(img, &points[i], &points[j]);
        }
    }
    ret
}

// とりあえず分岐点はないものとする（つまり，1つの特徴点から伸びている辺は2本だけとする）
// 点どうしが辺でつながっているかどうかを調べる．つながっていた場合はコストを返す，つながっていなかった場合は0を返す
fn is_adjacent(img: &Vec<Vec<usize>>, start: &Coordinate, goal: &Coordinate) -> usize {
    let x = start.x;
    let y = start.y;

    // Get branches
    for j in 0..3 {
        for i in 0..3 {
            if i != 1 && j != 1 {
                if img[j][i] == 1 {
                    let cost = is_adjacent(
                        img,
                        &Coordinate {
                            x: x + i - 1,
                            y: y + j - 1,
                        },
                        goal,
                    );
                    if cost == 0 {
                        return 0;
                    } else {
                        return cost + 1;
                    }
                }
            }
        }
    }
    0
}
