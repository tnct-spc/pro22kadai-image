// Vec[y][x]

fn conv_to_line(img: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut ret = Vec::new();
    let x_max = img[0].len();
    let y_max = img.len();

    for y in 0..y_max {
        for x in 0..x_max {
            ret[x + x_max * y] = img[y][x];
        }
    }
    ret
}

fn conv_from_line(img: &Vec<u8>, x_max: usize) -> Vec<Vec<u8>> {
    let mut ret = Vec::new();
    let y_max = img.len() / x_max;

    for y in 0..y_max {
        let mut ret_x = Vec::new();
        for x in 0..x_max {
            ret_x.push(img[y * x_max + x]);
        }
        ret.push(ret_x);
    }
    ret
}

// threshold: 輝度のしきい値（これ以上は白画素，これ以下は黒画素）
fn binarize(img: &mut Vec<Vec<u8>>, threshold: usize) {
    let mut p_src = conv_to_line(img);
    let x_max = img[0].len();
    let y_max = img.len();

    let mut lut = [0; 256];

    for i in threshold..256 {
        lut[i] = 255;
    }
    for i in 0..x_max * y_max {
        img[i / x_max][i % x_max] = lut[p_src[i] as usize];
    }
}
