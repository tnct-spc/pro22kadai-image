use crate::coordinate::{self, Coordinate};

struct ChainCode {
    start: Coordinate,
    current: Coordinate,
    chain: Vec<usize>,
}

impl ChainCode {
    fn next(&mut self) {
        let old_direction = self.chain[self.chain.len() - 1];
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
