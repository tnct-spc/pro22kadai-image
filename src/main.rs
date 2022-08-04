use convert::{file_to_vec, get_png_data};

mod binarization;
mod convert;

struct GRAYA {
    bright: u8,
    alpha: u8,
}

struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

struct PNG {
    image_header: IHDR,
    palette: PLTE,
    image_data: IDAT,
}

struct IHDR {
    length: u32,
    chunk_type: [char; 4],
    image_width: u32,
    image_height: u32,
    bit_depth: u8,
    color_type: u8,
    compress_method: u8,
    filter_method: u8,
    interlace_method: u8,
    crc: [u8; 4],
}

struct PLTE {
    length: u32,
    chunk_type: [char; 4],
    chunk_data: Vec<RGB>,
    crc: [u8; 4],
}

struct TRNS {
    length: u32,
    chunk_type: [char; 4],
    chunk_data: Vec<u8>,
    crc: [u8; 4],
}

struct IDAT {
    length: u32,
    chunk_type: [char; 4],
    chunk_data: Vec<u8>,
    crc: [u8; 4],
}

struct IEND {
    length: u32,
    chunk_type: [char; 4],
    crc: [u8; 4],
}

fn main() {
    let filename = String::from("./test.png");

    let data = file_to_vec(filename);

    let file_data = get_png_data(data);
}

fn gen_ary(x_max: usize, y_max: usize) -> Vec<Vec<u8>> {
    let mut ary = Vec::new();
    for y in 0..y_max {
        let mut x_ary = Vec::new();
        for x in 0..x_max {
            x_ary.push((y * x_max + x) as u8);
        }
        ary.push(x_ary);
    }
    ary
}

fn print_ary(ary: &Vec<Vec<u8>>) {
    let x_max = ary[0].len();
    let y_max = ary.len();

    for y in 0..y_max {
        print!("[{}", ary[y][0]);
        for x in 1..x_max {
            print!(", {}", ary[y][x]);
        }
        println!("],");
    }
}

fn print_line(ary: &Vec<u8>) {
    let l_max = ary.len();

    print!("[{}", ary[0]);
    for l in 1..l_max {
        print!(", {}", ary[l]);
    }
    println!("]");
}
