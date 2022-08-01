use convert::{file_to_vec, get_png_data};
use inflate::inflate_bytes;

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

fn get_binarized_data(file_data: PNG) -> Vec<usize> {
    // Encoded data line vector -> decoded pixel line vector
    let bit_depth = file_data.image_header.bit_depth;

    let pixel_data = inflate_bytes(&file_data.image_data.chunk_data).unwrap();

    match bit_depth {
        1 | 2 | 4 => ext_bit(pixel_data, bit_depth),
        8 => convert_vec_type(pixel_data),
        16 => join_byte(pixel_data),
        _ => vec![0],
    }
}

fn convert_vec_type(pixel_data: Vec<u8>) -> Vec<usize> {
    let mut ret = Vec::new();
    for p in pixel_data {
        ret.push(p as usize);
    }
    ret
}

fn join_byte(pixel_data: Vec<u8>) -> Vec<usize> {
    let l_max = pixel_data.len();

    let mut ret = Vec::new();

    let mut upper_digit: u16 = 0;

    for l in 0..l_max {
        match l % 2 {
            0 => upper_digit = pixel_data[l] as u16,
            1 => ret.push(((upper_digit << 8) + pixel_data[l] as u16) as usize),
            _ => {}
        }
    }
    ret
}

fn ext_bit(pixel_data: Vec<u8>, digit: u8) -> Vec<usize> {
    let mut mask: u8;
    let init_mask = generate_mask(digit);

    let mut ret = Vec::new();

    for d in pixel_data {
        mask = init_mask;
        for k in 0..(8 / digit) {
            ret.push(((d & mask) >> ((8 / digit - k - 1) * digit)) as usize);
            mask >> digit;
        }
    }
    ret
}

fn generate_mask(digit: u8) -> u8 {
    let mut mask: u8 = 0x80;
    for _i in 0..digit {
        mask |= mask >> 1;
    }
    mask
}
