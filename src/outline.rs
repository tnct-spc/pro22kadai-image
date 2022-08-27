use std::{cell::RefMut, cmp::min};

// outline：輪郭線を抽出する
// img：2値化した配列（黒画素：0，白画素：1）
pub fn outline(img: &mut Vec<Vec<usize>>) {
    let x_max = img[0].len();
    let y_max = img.len();

    for y in 1..y_max - 1 {
        for x in 1..x_max - 1 {
            if img[y][x] > 0
                && img[y - 1][x] > 0
                && img[y + 1][x] > 0
                && img[y][x - 1] > 0
                && img[y][x + 1] > 0
            {
                img[y][x] = 2;
            }
        }
    }
    for y in 0..y_max {
        for x in 0..x_max {
            img[y][x] %= 2;
        }
    }
    zero_padding(img);
}

fn zero_padding(img: &mut Vec<Vec<usize>>) {
    let x_max = img[0].len();
    let y_max = img.len();

    for y in 0..y_max {
        img[y][0] = 0;
        img[y][x_max - 1] = 0;
    }

    for x in 0..x_max {
        img[0][x] = 0;
        img[y_max - 1][x] = 0;
    }
}

pub fn labelling(img: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let y_max = img.len();
    let x_max = img[0].len();

    let mut ret = vec![vec![0; x_max]; y_max];

    let mut lut = vec![0];

    for y in 1..y_max - 1 {
        for x in 1..x_max - 1 {
            if img[y][x] == 1 {
                // 白画素だった場合
                if img[y][x - 1] + img[y - 1][x - 1] + img[y - 1][x] + img[y - 1][x + 1] == 0 {
                    // 左，左上，上，右上の画素がすべて0の場合
                    // 最後に割り振った番号+1（=lutの要素数）を割り振る
                    ret[y][x] = lut.len();
                    lut.push(lut.len());
                } else {
                    // 左上，上，右上，左のどれかが0じゃない
                    // 最小のラベルを探す
                    let mut s = lut.len();
                    if img[y][x - 1] > 0 {
                        // 左側の画素が0じゃない場合，左側の画素のラベルを候補にする
                        s = img[y][x - 1];
                    }
                    let j = y - 1;
                    let i = x - 1;
                    if img[j][i] > 0 {
                        // 左上の画素が0じゃない場合
                        if img[j][i] < s {
                            // 左上の画素のラベルが候補より小さい場合，ラベル候補とlutを更新する

                            s = img[j][i];
                        }
                    }
                }
            }
        }
    }
    ret
}
