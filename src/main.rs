use png_converter::get_pixel_data;

mod binarization;
mod png_converter;

fn main() {
    let filename = "./ThinkPhone.png";

    let pixel_data = get_pixel_data(filename).unwrap();

    print_ary(&pixel_data);
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

// PNGをバイト配列で読み込む
// 画素部分のデータを取り出す
// データをinflateする
// データを2次元vectorにする
// unfilterする
