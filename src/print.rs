use crate::adjacent_points::{self, AdjacentPoints};
use crate::coordinate::Coordinate;

const BLACK: &str = " ";
const WHITE: &str = "*";

pub fn print_vec(ary: &Vec<Vec<usize>>) {
    let y_max = ary.len();
    let x_max = ary[0].len();

    for y in 0..y_max {
        print!("[{:02}", ary[y][0]);
        for x in 1..x_max {
            print!(", {}", ary[y][x]);
        }
        println!("],");
    }
}

pub fn print_ptn(ary: &Vec<Vec<usize>>) {
    for l in ary {
        for d in l {
            print!("{}", if *d == 0 { BLACK } else { WHITE });
        }
        println!();
    }
    println!();
}

pub fn print_adjacent_matrix(adjacent_matrix: &Vec<Vec<usize>>) {
    for l in adjacent_matrix {
        for d in l {
            print!("{:2}", *d);
        }
        println!();
    }
    println!();
}

pub fn print_points(points: &Vec<Coordinate>) {
    for p in points {
        println!("{}", *p);
    }
}

pub fn print_adjacent_points(adjacent_points: &Vec<AdjacentPoints>) {
    for p in adjacent_points {
        println!("{}", p);
    }
}
