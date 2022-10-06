use lambda_runtime::{service_fn, Error, LambdaEvent};
use serde_json::{json, Value};

use crate::main;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (_event, _context) = event.into_parts();

    let encoded_img = get_base64_from_url(url);
    let img = get_gray_data_from_base64(encoded_img);
    let mut img = binarize(img);
    outline(&mut img);
    noize_erase(&mut img);

    let points = pick_corner_point(&img);

    let adjacent_matrix = get_adjacent_matrix(&points, &img);

    let (points, adjacent_matrix) = merge_points(points, adjacent_matrix);

    Ok(vec_to_json(points, adjacent_matrix))
}
