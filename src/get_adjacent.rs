use crate::coordinate::Coordinate;

struct ChainCode {
    start: Coordinate,
    current: Coordinate,
    chain: Vec<usize>,
}

impl ChainCode {
    // 駒を進める
    fn next(&mut self, img: &Vec<Vec<usize>>) {
        let old_direction = self.chain[self.chain.len() - 1].abs_diff(4);

        for d in 0..8 {
            if d != old_direction {
                if is_pixel_white(self.current, img, d) {}
            }
        }
    }
}

const D: [[isize; 2]; 8] = [
    [-1, 1],
    [0, 1],
    [1, 1],
    [1, 0],
    [1, -1],
    [0, -1],
    [-1, -1],
    [-1, 0],
];

fn next_pixel(current_point: Coordinate) {}

pub fn get_adjacent_matrix(_points: &Vec<Coordinate>, _img: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    vec![vec![0]]
}

fn get_beside_coordinate(current: Coordinate, direction: usize) -> Coordinate {
    let x = current.x as isize;
    let y = current.y as isize;

    let x = x + D[direction][0];
    let y = y + D[direction][1];

    Coordinate {
        x: x as usize,
        y: y as usize,
    }
}

fn is_pixel_white(current: Coordinate, img: &Vec<Vec<usize>>, direction: usize) -> bool {
    let new_coord = get_beside_coordinate(current, direction);

    if img[new_coord.y][new_coord.x] > 0 {
        true
    } else {
        false
    }
}
