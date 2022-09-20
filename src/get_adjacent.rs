use crate::coordinate::Coordinate;

struct Direction {
    x: isize,
    y: isize,
}

struct ChainCode {
    start: Coordinate,
    current: Coordinate,
    chain: Vec<usize>,
}

impl ChainCode {
    fn init(start_index: usize, start_coordinate: Coordinate) -> ChainCode {
        ChainCode {
            start: start_coordinate,
            current: start_coordinate,
            chain: vec![start_index],
        }
    }
    // 駒を進める
    fn next(&mut self, img: &Vec<Vec<usize>>) {
        let old_direction = self.chain[self.chain.len() - 1].abs_diff(4);

        for d in 0..8 {
            if d != old_direction {
                if is_pixel_white(self.current, img, d) {
                    self.current = get_beside_coordinate(self.current, d);
                    return;
                }
            }
        }
    }
}

const D: [Direction; 8] = [
    Direction { x: -1, y: 1 },
    Direction { x: 0, y: 1 },
    Direction { x: 1, y: 1 },
    Direction { x: 1, y: 0 },
    Direction { x: 1, y: -1 },
    Direction { x: 0, y: -1 },
    Direction { x: -1, y: -1 },
    Direction { x: -1, y: 0 },
];

pub fn get_adjacent_matrix(points: &Vec<Coordinate>, img: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let y_max = img.len();
    let x_max = img[0].len();
    let ret = vec![vec![0; x_max]; y_max];

    for (i, p) in points.iter().enumerate() {
        let mut chain_code = ChainCode::init(i, *p);
    }
    ret
}

fn get_beside_coordinate(current: Coordinate, direction: usize) -> Coordinate {
    let x = current.x as isize;
    let y = current.y as isize;

    let x = x + D[direction].x;
    let y = y + D[direction].y;

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

fn search_points(target: Coordinate, points: &Vec<Coordinate>) -> usize {
    for (i, p) in points.iter().enumerate() {
        if *p == target {
            return i;
        }
    }
    points.len()
}
