// Vec[y][x]

pub fn conv_to_line(img: Vec<Vec<usize>>) -> Vec<usize> {
    let mut ret = Vec::new();
    let x_max = img[0].len();
    let y_max = img.len();

    for y in 0..y_max {
        for x in 0..x_max {
            ret.push(img[y][x]);
        }
    }
    ret
}

pub fn conv_from_line(img: Vec<usize>, x_max: usize) -> Vec<Vec<usize>> {
    let mut ret = Vec::new();
    let y_max = img.len() / x_max;

    if x_max * y_max == img.len() {
        for y in 0..y_max {
            let mut ret_x = Vec::new();
            for x in 0..x_max {
                ret_x.push(img[y * x_max + x]);
            }
            ret.push(ret_x);
        }
        return ret;
    }
    vec![vec![0]]
}

// threshold: 輝度のしきい値（これ以上は白画素，これ以下は黒画素）
pub fn binarize(img: &mut Vec<Vec<usize>>, threshold: usize) {
    let p_src = conv_to_line(img.clone());
    let x_max = img[0].len();
    let y_max = img.len();

    let mut lut = [0; 256];

    for i in threshold..256 {
        lut[i] = 1;
    }
    for i in 0..x_max * y_max {
        img[i / x_max][i % x_max] = lut[p_src[i] as usize];
    }
}

// 判別分析法の閾値を求める
fn get_threshold(img: &Vec<Vec<u8>>) -> usize {
    let histgram = get_histgram(&img);

    let mut threshold = 0;
    let mut t;
    let mut t_max = 0.0;

    for i in 0..256 {
        t = split_histgram(&histgram, i);
        if t_max < t {
            t_max = t;
            threshold = i;
        }
    }
    threshold
}

fn get_histgram(img: &Vec<Vec<u8>>) -> [usize; 256] {
    let mut histgram = [0; 256];

    let x_max = img[0].len();
    let y_max = img.len();

    for y in 0..y_max {
        for x in 0..x_max {
            histgram[img[y][x] as usize] += 1;
        }
    }
    histgram
}

fn split_histgram(histgram: &[usize; 256], threshold: usize) -> f64 {
    let t = threshold as f64;

    let mut sum_black: f64 = 0.0;
    let mut sum_white: f64 = 0.0;
    let ave_black: f64;
    let ave_white: f64;

    for i in 0..threshold {
        sum_black += histgram[i] as f64;
    }
    ave_black = sum_black / t;
    for i in threshold..256 {
        sum_white += histgram[i] as f64;
    }
    ave_white = sum_white / (256.0 - t);

    sum_black * sum_white * (ave_white - ave_black) * (ave_white - ave_black)
}
