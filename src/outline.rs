// outline：輪郭線を抽出する
// img：2値化した配列（黒画素：0，白画素：1）
fn outline(img: &mut Vec<Vec<u8>>) {
    let x_max = img[0].len();
    let y_max = img.len();

    for y in 0..y_max {
        for x in 0..x_max {
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
