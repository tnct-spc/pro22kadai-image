use serde::Deserialize;
use serde_json::{json, Value};

use axum::{
    extract::Query,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use binarization::binarize;
use coordinate::Coordinate;
use corner_detector::{noize_erase, pick_corner_point, print_coordinates};
use get_adjacent::get_adjacent_matrix;
use labelling::{get_adjacent_matrix_from_label, labelling};
use merge_points::merge_points;
use outline::{outline, zero_padding};
use png_reader::{
    get_base64_from_url, get_color_data_from_base64, get_color_data_from_filename,
    get_gray_data_from_base64, get_gray_data_from_filename, png_to_base64,
};
use print::{
    print_adjacent_matrix, print_adjacent_points, print_asterism, print_points, print_ptn,
    print_vec,
};
use vec_to_json::vec_to_json;

mod binarization;
mod coordinate;
mod corner_detector;
mod get_adjacent;
mod labelling;
mod merge_points;
mod outline;
mod png_reader;
mod print;
mod vec_to_json;

// fn main() {
//     let encoded_img = png_reader::png_to_base64("images/daruma_padd_ex.png");
//     // adjacent_matrix_test(encoded_img);
//     let res = get_points(encoded_img);

//     println!("{}", res);

//     // let res = binarization_test(encoded_img);
//     // print_ptn(&res);
// }

fn binarization_test(encoded_img: String) -> Vec<Vec<usize>> {
    zero_padding(binarize(get_gray_data_from_base64(encoded_img)))
}

fn adjacent_matrix_test(encoded_img: String) {
    let mut img = zero_padding(binarize(get_gray_data_from_base64(encoded_img)));
    outline(&mut img);
    noize_erase(&mut img);

    let points = pick_corner_point(&img);

    let labelled_img = labelling(&img);
    let adjacent_matrix = get_adjacent_matrix_from_label(&points, &labelled_img);

    // print_adjacent_points(&points, &adjacent_matrix);
}

async fn get_points(encoded_img: String) -> Value {
    // let img = get_gray_data_from_base64(encoded_img);
    // let img = binarize(img);
    let mut img = zero_padding(binarize(get_gray_data_from_base64(encoded_img)));
    outline(&mut img);
    noize_erase(&mut img);

    let points = pick_corner_point(&img);

    // let labelled_img = labelling(&img);
    // let adjacent_matrix = get_adjacent_matrix_from_label(&points, &labelled_img);

    let adjacent_matrix = get_adjacent_matrix(&points, &img);

    let (points, adjacent_matrix) = merge_points(points, adjacent_matrix);

    // print_adjacent_points(&points, &adjacent_matrix);

    vec_to_json(points, adjacent_matrix)
}

#[derive(Deserialize)]
struct PostParamater {
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
    let encoded_img = params.img;

    let res = get_points(encoded_img).await;

    Json(res)
}
