use binarization::binarize;
use coordinate::Coordinate;
use corner_detector::{noize_erase, pick_corner_point, print_coordinates};
use get_adjacent::get_adjacent_matrix;
use get_base64_from_url::get_base64_from_url;
use outline::outline;
use png_reader::get_gray_data_from_filename;
use png_reader::{
    get_color_data_from_base64, get_color_data_from_filename, get_gray_data_from_base64,
    png_to_base64,
};
use print::print_points;
use print::{print_adjacent_matrix, print_ptn, print_vec};
use vec_to_json::vec_to_json;

use crate::print::print_adjacent_points;

mod adjacent_points;
mod binarization;
mod coordinate;
mod corner_detector;
mod get_adjacent;
mod get_base64_from_url;
// mod merge_points;
mod merge_points_2;
mod outline;
mod png_reader;
mod print;
mod vec_to_json;

fn main() {
    let img = get_gray_data_from_filename("ThinkPhone.png");
    let mut img = binarize(img);
    outline(&mut img);
    noize_erase(&mut img);

    let points = pick_corner_point(&img);
    println!("Points: ");
    print_points(&points);

    let mut adjacent_points = get_adjacent_matrix(&points, &img);
    adjacent_points.sort();
    print_adjacent_points(&adjacent_points);
}
