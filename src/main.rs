use binarization::binarize;
use coordinate::Coordinate;
use corner_detector::{pick_corner_point, print_coordinates};
use get_adjacent::get_adjacent_matrix;
use outline::outline;
use png_reader::{
    get_gray_data_from_base64, get_pixel_data_from_base64, get_pixel_data_from_filename,
    png_to_base64,
};
use vec_to_json::vec_to_json;

mod binarization;
mod coordinate;
mod corner_detector;
mod get_adjacent;
mod outline;
mod png_reader;
mod vec_to_json;

const BLACK: &str = " ";
const WHITE: &str = "*";

fn main() {
    test_grey_scale();
}

fn test_grey_scale() {
    let filename = "./InCaseOfFire2.PNG";

    let filedata = png_to_base64(filename);

    let image_data = get_gray_data_from_base64(filedata);

    let mut image_pixels = binarize(image_data);

    outline(&mut image_pixels);

    // print_ptn(&image_pixels);

    println!("Start to pick points");
    let points = pick_corner_point(&image_pixels);
    print_coordinates(&points);

    println!("Start to get adjacent matrix");
    let adjacent_matrix = get_adjacent_matrix(&points, &image_pixels);
    // print_vec(&adjacent_matrix);
    println!("Finished to get adjacent matrix.");
}

fn test_find_points() {
    let filename = "./ThinkPhone.png";

    let file_data = png_to_base64(filename);

    let ret = get_pixel_data_from_base64(file_data);

    // let ret = get_pixel_data_from_filename(filename);
    let red_pixels = ret.0;
    let green_pixels = ret.1;
    let blue_pixels = ret.2;
    let alpha_pixels = ret.3;

    let mut red_pixels = binarize(red_pixels);
    let mut green_pixels = binarize(green_pixels);
    let mut blue_pixels = binarize(blue_pixels);
    let mut alpha_pixels = binarize(alpha_pixels);

    outline(&mut red_pixels);
    outline(&mut green_pixels);
    outline(&mut blue_pixels);
    outline(&mut alpha_pixels);

    let marged_pixels = marge_vec(red_pixels, green_pixels, blue_pixels, alpha_pixels);

    println!("Start to pick up points");
    let points = pick_corner_point(&marged_pixels);

    print_coordinates(&points);
    println!("{} points are found.", points.len());

    println!("Start to get adjacent matrix");
    let adjacent_matrix = get_adjacent_matrix(&points, &marged_pixels);
    print_vec(&adjacent_matrix);

    // let json = vec_to_json(&points);
    // println!("{}", json);

    println!("{} points are found.", points.len());
}

fn print_vec(ary: &Vec<Vec<usize>>) {
    let x_max = ary[0].len();
    let y_max = ary.len();

    for y in 0..y_max {
        print!("[{:02}", ary[y][0]);
        for x in 1..x_max {
            print!(", {}", ary[y][x]);
        }
        println!("],");
    }
}

fn draw_rectangle(ary: &mut Vec<Vec<usize>>, points: &Vec<Coordinate>) {
    let r = 5;
    let s = r / 2 + 1;
    for p in points {
        for j in 0..r {
            for i in 0..r {
                let x = (*p).x;
                let y = (*p).y;

                if i == 0 || i == r - 1 || j == 0 || j == r - 1 {
                    ary[y - s + j][x - s + i] = 1;
                }
            }
        }
    }
}

fn print_ptn(ary: &Vec<Vec<usize>>) {
    let x_max = ary[0].len();
    // let x_max = 500;
    let y_max = ary.len();
    // let y_max = 500;

    for y in 0..y_max {
        for x in 0..x_max {
            print!("{}", if ary[y][x] == 0 { BLACK } else { WHITE });
            // print!("{:2}", ary[y][x]);
        }
        println!();
    }
    println!();
}

fn marge_vec(
    r: Vec<Vec<usize>>,
    g: Vec<Vec<usize>>,
    b: Vec<Vec<usize>>,
    a: Vec<Vec<usize>>,
) -> Vec<Vec<usize>> {
    let mut ret = Vec::new();

    let x_max = r[0].len();
    let y_max = r.len();
    for y in 0..y_max {
        let mut ret_x = Vec::new();
        for x in 0..x_max {
            ret_x.push(
                if r[y][x] > 0 || g[y][x] > 0 || b[y][x] > 0 || a[y][x] > 0 {
                    1
                } else {
                    0
                },
            );
        }
        ret.push(ret_x);
    }
    ret
}
