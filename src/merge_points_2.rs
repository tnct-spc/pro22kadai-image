use crate::adjacent_points::AdjacentPoints;
use crate::coordinate::Coordinate;
use crate::get_adjacent::euclid_distance;

const S: usize = 3; // 頂点間のマンハッタン距離がこれ以下だった場合は問答無用で結合

// limit: 頂点の数の上限
fn merge_points(points: Vec<Coordinate>, adjacent_matrix: Vec<AdjacentPoints>, limit: usize) {
    for p in adjacent_matrix {
        if p.distance == 0 {
            continue;
        } else if p.distance <= S {
        }
    }
}
