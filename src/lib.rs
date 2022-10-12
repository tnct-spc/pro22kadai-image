#[macro_use]
extern crate rutie;

use rutie::{Module, Object, RString};

use binarization::binarize;
use coordinate::Coordinate;
use corner_detector::{noize_erase, pick_corner_point, print_coordinates};
use get_adjacent::get_adjacent_matrix;
use merge_points::merge_points;
use outline::outline;
use png_reader::{
    get_base64_from_url, get_color_data_from_base64, get_color_data_from_filename,
    get_gray_data_from_base64, get_gray_data_from_filename, png_to_base64,
};
// use print::print_adjacent_points;
// use print::print_points;
// use print::{print_adjacent_matrix, print_ptn, print_vec};
use vec_to_json::vec_to_json;

mod binarization;
mod coordinate;
mod corner_detector;
mod get_adjacent;
mod merge_points;
mod outline;
mod png_reader;
// mod print;
mod vec_to_json;

module!(GetPoints);

methods!(
    GetPoints,
    _rtself,
    fn pub_get_points(encoded_img: RString) -> RString {
        let img = get_gray_data_from_base64(encoded_img.unwrap().to_string());
        let mut img = binarize(img);
        outline(&mut img);
        noize_erase(&mut img);

        let points = pick_corner_point(&img);

        let adjacent_matrix = get_adjacent_matrix(&points, &img);

        let (points, adjacent_matrix) = merge_points(points, adjacent_matrix);

        RString::from(vec_to_json(points, adjacent_matrix))
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_GetPoints() {
    Module::new("GetPoints").define(|module| module.def_self("get_points", pub_get_points));
}
