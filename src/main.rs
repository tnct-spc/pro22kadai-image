use binarization::binarize;
use image::GenericImageView;
use outline::outline;
use png_reader::get_pixel_data;

mod binarization;
mod outline;
mod png_reader;
/* mod png_converter; */
/* mod zlib; */

fn main() {
    let filename = "./ThinkPhone.png";

    let (red_pixels, green_pixels, blue_pixels, alpha_pixels) = get_pixel_data(filename);

    let mut red_bin = binarize(red_pixels);
    let mut green_bin = binarize(green_pixels);
    let mut blue_bin = binarize(blue_pixels);
    let mut alpha_bin = binarize(alpha_pixels);

    outline(&mut red_bin);
    outline(&mut green_bin);
    outline(&mut blue_bin);
    outline(&mut alpha_bin);

    print_ptn(&red_bin);
    print_ptn(&green_bin);
    print_ptn(&blue_bin);
    print_ptn(&alpha_bin);

    let pixel_data = marge_vec(red_bin, green_bin, blue_bin, alpha_bin);

    print_ptn(&pixel_data);
}

fn print_ary(ary: &Vec<Vec<usize>>) {
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

fn print_ptn(ary: &Vec<Vec<usize>>) {
    for y in ary {
        for x in y {
            print!("{}", if *x == 0 { "  " } else { "HH" });
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
