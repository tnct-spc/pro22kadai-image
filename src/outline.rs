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
}

pub fn zero_padding(img: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let y_max = img.len();
    let x_max = img[0].len();

    let mut ret = Vec::new();

    // 上の白ピクセルの行を作る
    let mut ret_line = vec![0, 0];
    for x in 0..x_max {
        ret_line.push(0);
    }
    ret.push(ret_line);

    // 中の部分を作る
    for y in 0..y_max {
        let mut ret_line = vec![0];
        for x in 0..x_max {
            ret_line.push(img[y][x]);
        }
        ret_line.push(0);
        ret.push(ret_line);
    }

    // 下の白ピクセルの行を作る
    let mut ret_line = vec![0, 0];
    for x in 0..x_max {
        ret_line.push(0);
    }
    ret.push(ret_line);

    ret
}
