use crate::get_adjacent::distance;
use crate::Coordinate;

pub fn labelling(img: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let y_max = img.len();
    let x_max = img[0].len();

    let mut ret_img = vec![vec![0; x_max]; y_max];

    let mut lut = vec![0, 1];

    // 画素をラスタスキャンする
    for y in 0..y_max {
        for x in 0..x_max {
            if img[y][x] > 0 {
                if img[y][x - 1] == 0
                    && img[y - 1][x - 1] == 0
                    && img[y - 1][x] == 0
                    && img[y - 1][x + 1] == 0
                {
                    // 左，左上，上，右上のラベルが0の場合は最後に割り振った番号+1のラベル番号を割り振る（=lutの長さを割り振る）
                    let l = lut.len();
                    ret_img[y][x] = l;
                    lut.push(l);
                } else {
                    // それ以外の場合
                    // 左上，上，右上のラベルの中で最小の番号を割り振る
                    let mut unused_labels = Vec::new(); // 使わなかった哀れなラベルたち
                    let mut l = img[y][x - 1]; // とりあえず左を代入する（0でも）
                    if l == 0 || img[y - 1][x - 1] < l {
                        // 左が0だったor左上が左より小さいなら左上を代入する
                        if l > 0 {
                            unused_labels.push(l);
                        }
                        l = img[y - 1][x - 1];
                    }
                    if l == 0 || img[y - 1][x] < l {
                        // 左上が0だったor上が左上より小さいなら上を(ry
                        if l > 0 {
                            unused_labels.push(l);
                        }
                        l = img[y - 1][x];
                    }
                    if l == 0 || img[y - 1][x + 1] < l {
                        // 上が0だったor右上が上より(ry
                        if l > 0 {
                            unused_labels.push(l);
                        }
                        l = img[y - 1][x + 1];
                    }
                    ret_img[y][x] = l; // ラベルを割り振る
                    for u in unused_labels {
                        // 使わなかったラベル番号に対応するlutを最小の番号にする
                        lut[u] = l;
                    }
                }
            }
        }
    }
    // lutを使って置き換える
    for y in 0..y_max {
        for x in 0..x_max {
            ret_img[y][x] = lut[ret_img[y][x]];
        }
    }
    ret_img
}

pub fn get_adjacent_matrix_from_label(
    points: &Vec<Coordinate>,
    labelled_img: &Vec<Vec<usize>>,
) -> Vec<Vec<usize>> {
    let points_count = points.len();

    let mut ret = vec![vec![0; points_count]; points_count];

    for j in 0..points_count {
        for i in j + 1..points_count {
            if labelled_img[points[i].y][points[i].x] == labelled_img[points[j].y][points[j].x] {
                ret[i][j] = distance(points[i], points[j]);
                ret[j][i] = distance(points[i], points[j]);
            }
        }
    }
    ret
}
