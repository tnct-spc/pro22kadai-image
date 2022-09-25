use crate::coordinate::Coordinate;

struct Direction {
x: isize,
                                        y: isize,
}

impl Direction {
    fn new(x: isize, y: isize) -> Direction {
        Direction { x, y }
    }
}

impl Coordinate {
    fn get_beside_coordinate(&self, direction: Direction) -> Coordinate {
        let x = self.x as isize;
        let y = self.y as isize;

        let x = x + direction.x;
        let y = y + direction.y;

        Coordinate {
            x: x as usize,
            y: y as usize,
        }
    }
}

const D: [Direction; 8] = [
    Direction { x: -1, y: 1 },
    Direction { x: -1, y: 0 },
    Direction { x: -1, y: -1 },
    Direction { x: 0, y: -1 },
    Direction { x: 1, y: -1 },
    Direction { x: 1, y: 0 },
    Direction { x: 1, y: 1 },
    Direction { x: 0, y: 1 },
];

struct ChainCode {
    start: Coordinate,
    goal: Coordinate,
    cost: usize,
    old_direction: usize,
}

impl ChainCode {
    fn new(start: usize, points: &Vec<Coordinate>) -> ChainCode {
        ChainCode {
            start: points[start],
            goal: points[start],
            cost: 0,
            old_direction: 8,
        }
    }
    fn next(&self) -> Option<ChainCode> {
        for d in 0..8 {
            // 前のピクセルに戻らないようにする
            if self.old_direction.abs_diff(4) != d {}
        }
        return None;
    }
}

// Receive image data and points data, and return adjacent matrix.
pub fn get_adjacent_matrix(img: &Vec<Vec<usize>>, points: &Vec<Coordinate>) -> Vec<Vec<usize>> {
    let points_count = points.len();
    let mut picked_point_count = 0;

    let mut ret = vec![vec![0; points_count]; points_count];

    let mut point_index = 0;

    // Pick up first point
    let mut start = 0;

    while picked_point_count == points_count {
        let (goal, cost) = get_goal_pixels(start, points);

        ret[start][goal] = cost;
        ret[goal][start] = cost;

        start = goal;
    }
    ret
}

fn is_pixel_white(target: Coordinate, img: &Vec<Vec<usize>>) -> bool {
    img[target.y][target.x] > 0
}

fn get_goal_pixels(start: usize, points: &Vec<Coordinate>) -> (usize, usize) {
    let mut chain_code = ChainCode::new(start, points);
    let ret = 0;

    (ret, chain_code.cost)
}

fn search_pixel(target: Coordinate, points: &Vec<Coordinate>) -> isize {
    for (i, p) in points.iter().enumerate() {
        if target == *p {
            return i as isize;
        }
    }
    -1
}
