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

const D: [Direction; 8] = [
    Direction::new(-1, 1),
    Direction::new(-1, 0),
    Direction::new(-1, -1),
    Direction::new(0, -1),
    Direction::new(1, -1),
    Direction::new(1, 0),
    Direction::new(1, 1),
    Direction::new(0, 1),
];

struct ChainCode {
    start: usize,
    goal: usize,
    cost: usize,
    old_direction: usize,
}

impl ChainCode {
    fn new(start: usize) -> ChainCode {
        ChainCode {
            start,
            goal: start,
            cost: 0,
            old_direction: 8,
        }
    }
    fn next(&self) -> ChainCode {
        for d in 0..8 {
            if self.old_direction != d {
                
            }
        }
        ChainCode::new(0)
    }
}

// Receive image data and points data, and return adjacent matrix.
pub fn get_adjacent_matrix(img: &Vec<Vec<usize>>, points: &Vec<Coordinate>) -> Vec<Vec<usize>> {
    let points_count = points.len();
    let mut picked_point_count = 0;

    let mut ret = vec![vec![0; points_count]; points_count];

    let mut point_index = 0;

    // Pick up first point
    let mut p = points[0];

    while picked_point_count == points_count {
        let chain_code = get_chain_code(p);

        ret[chain_code.start][chain_code.goal] = chain_code.cost;
        ret[chain_code.goal][chain_code.start] = chain_code.cost;

        p = points[chain_code.goal];
    }
    ret
}

fn get_chain_code(start: usize) -> ChainCode {
    let mut chain_code = ChainCode::new(start);

    ChainCode::new(0)
}

fn get_beside_coordinate(coordinate: Coordinate, direction: Direction) -> Coordinate {
    let x = coordinate.x as isize;
    let y = coordinate.y as isize;

    let x = x + direction.x;
    let y = y + direction.y;

    Coordinate {
        x: x as usize,
        y: y as usize,
    }
}
