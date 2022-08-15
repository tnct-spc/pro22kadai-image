use binarization::binarize;
use corner_detector::{pick_corner_point, print_coordinates, Coordinate};
use outline::outline;
use png_reader::get_pixel_data;
use vec_to_json::vec_to_json;

mod binarization;
mod corner_detector;
mod outline;
mod png_reader;
mod vec_to_json;

const BLACK: &str = "　";
const WHITE: &str = "鬱";

fn main() {
    let filename = "./ThinkPhone.png";

    let (red_pixels, green_pixels, blue_pixels, alpha_pixels) = get_pixel_data(filename);

    let mut red_pixels = binarize(red_pixels);
    let mut green_pixels = binarize(green_pixels);
    let mut blue_pixels = binarize(blue_pixels);
    let mut alpha_pixels = binarize(alpha_pixels);

    outline(&mut red_pixels);
    outline(&mut green_pixels);
    outline(&mut blue_pixels);
    outline(&mut alpha_pixels);

    let mut marged_pixels = marge_vec(red_pixels, green_pixels, blue_pixels, alpha_pixels);

    let points = pick_corner_point(&marged_pixels);

    // draw_rectangle(&mut marged_pixels, &points);
    // print_ptn(&marged_pixels);

    // print_coordinates(&points);

    let json = vec_to_json(&points);
    println!("{}", json);

    println!("{} points are found.", points.len());
}

fn print_vec(ary: &Vec<Vec<usize>>) {
    let x_max = ary[0].len();
    let y_max = ary.len();

    for y in 0..y_max {
        print!("[{:02x}", ary[y][0]);
        for x in 1..x_max {
            print!(", {:02x}", ary[y][x]);
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
            // print!(("{:3x}",ary[y][x]));
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
