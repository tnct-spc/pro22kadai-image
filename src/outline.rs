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
