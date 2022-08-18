use crate::corner_detector::Coordinate;

pub fn get_adjacent_matrix(img: &Vec<Vec<usize>>, points: &Vec<Coordinate>) -> Vec<Vec<usize>> {
    let s = points.len();

    let mut ret = vec![vec![0; s]; s];

    for j in 0..s {
        for i in j..s {
            if is_adjacent(img, &points[i], &points[j]) {
                ret[i][j] = get_euclid_distance(&points[i], &points[j]);
                ret[j][i] = get_euclid_distance(&points[i], &points[j]);
            } else {
                ret[i][j] = 0;
                ret[j][i] = 0;
            }
        }
    }
    ret
}

fn is_adjacent(img: &Vec<Vec<usize>>, p1: &Coordinate, p2: &Coordinate) -> bool {
    let x_max = img[0].len();
    let y_max = img.len();

    let mut x = p1.x;
    let mut y = p1.y;

    loop {}

    false
}

fn get_euclid_distance(p1: &Coordinate, p2: &Coordinate) -> usize {
    let x1 = p1.x as f64;
    let y1 = p1.y as f64;
    let x2 = p2.x as f64;
    let y2 = p2.y as f64;

    ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt() as usize
}
