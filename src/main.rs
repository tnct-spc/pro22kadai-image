use axum::{extract::Query, response::IntoResponse, routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use binarization::binarize;
use coordinate::Coordinate;
use corner_detector::{noize_erase, pick_corner_point, print_coordinates};
use get_adjacent::get_adjacent_matrix;
use image_reader::{
    get_base64_from_url, get_color_data_from_base64, get_color_data_from_filename,
    get_gray_data_from_base64, get_gray_data_from_filename, png_to_base64,
};
use merge_points::merge_points;
use outline::{outline, zero_padding};
// use print::print_adjacent_points;
// use print::print_points;
// use print::{print_adjacent_matrix, print_ptn, print_vec};
use print::{print_adjacent_matrix, print_points};
use vec_to_json::vec_to_json;

mod binarization;
mod coordinate;
mod corner_detector;
mod get_adjacent;
mod image_reader;
mod merge_points;
mod outline;
mod print;
mod vec_to_json;

// fn main() {
//     let encoded_img = image_reader::png_to_base64("images/nut_logo.gif");
//     let res = get_points(encoded_img);

//     println!("{}", res);
// }

async fn get_points(encoded_img: String) -> Value {
    let img = get_gray_data_from_base64(encoded_img);
    let mut img = binarize(img);
    // let mut img = zero_padding(img);
    // println!("height: {}, width: {}", img.len(), img[0].len());
    // let mut img = zero_padding(binarize(get_gray_data_from_base64(encoded_img)));
    outline(&mut img);
    noize_erase(&mut img);

    let points = pick_corner_point(&img);
    // print_points(&points);

    let adjacent_matrix = get_adjacent_matrix(&points, &img);
    // print_adjacent_matrix(&adjacent_matrix);

    let (points, adjacent_matrix) = merge_points(points, adjacent_matrix);

    vec_to_json(points, adjacent_matrix)
}

#[derive(Deserialize)]
pub struct PostParamater {
    img: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(func));
    let app = lambda_http::tower::ServiceBuilder::new()
        .layer(axum_aws_lambda::LambdaLayer::default())
        .service(app);

    lambda_http::run(app).await.unwrap();
}

async fn func(Json(params): Json<PostParamater>) -> impl IntoResponse {
    // Json(get_points(params.img).await)
    Json(get_points(params.img).await)
}
