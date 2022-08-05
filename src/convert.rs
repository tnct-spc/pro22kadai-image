use inflate::inflate_bytes;
use std::fs::File;
use std::io::Read;

use crate::binarization::conv_from_line;

const TYPE_IHDR: &[char; 4] = &['I', 'H', 'D', 'R'];
const TYPE_PLTE: &[char; 4] = &['P', 'L', 'T', 'E'];
const TYPE_TRNS: &[char; 4] = &['t', 'R', 'N', 'S'];
const TYPE_IDAT: &[char; 4] = &['I', 'D', 'A', 'T'];
const TYPE_IEND: &[char; 4] = &['I', 'E', 'N', 'D'];

struct GRAYA {
    bright: u8,
    alpha: u8,
}

struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}
struct RGBA {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

struct DATACHUNK {
    length: u32,
    chunk_type: [char; 4],
    chunk_data: Vec<u8>,
    // crc: [u8; 4],
}

pub struct PNG {
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
    // crc: [u8; 4],
}

struct PLTE {
    length: u32,
    chunk_type: [char; 4],
    chunk_data: Vec<RGB>,
    // crc: [u8; 4],
}

struct TRNS {
    length: u32,
    chunk_type: [char; 4],
    chunk_data: Vec<u8>,
    // crc: [u8; 4],
}

struct IDAT {
    length: u32,
    chunk_type: [char; 4],
    chunk_data: Vec<u8>,
    // crc: [u8; 4],
}

struct IEND {
    length: u32,
    chunk_type: [char; 4],
    // crc: [u8; 4],
}

// Open file and convert to Vec<u8>
pub fn file_to_vec(filename: String) -> Vec<u8> {
    let mut file = File::open(filename).expect("can't open a file!");

    let mut data = Vec::new();

    file.read_to_end(&mut data).expect("can't read data!");

    data
}

// Determine if the specified file is PNG or not
fn is_file_png(data: &Vec<u8>) -> bool {
    let png_signature = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
    let mut is_png = false;

    for i in 0..8 {
        is_png &= data[i] == png_signature[i];
    }
    is_png
}

fn slice(data: &Vec<u8>, offset: usize, size: usize) -> Vec<u8> {
    let mut ret = Vec::new();

    for i in 0..size {
        ret.push(data[i + offset]);
    }
    ret
}

fn verify_chunk_type(data: &[char; 4], chunk_type: &[char; 4]) -> bool {
    let mut ret = true;

    for i in 0..4 {
        ret &= data[i] == chunk_type[i];
    }
    ret
}

fn byte_to_u32(data: &Vec<u8>, offset: usize) -> u32 {
    let mut ret = 0;
    let data_count = data.len();

    for l in 4..0 {
        ret += (data[data_count - l + offset] as usize * 256 ^ l) as u32;
    }
    ret
}

fn get_chunk_type(data: &Vec<u8>, offset: usize) -> [char; 4] {
    let mut ret: [char; 4] = ['\0'; 4];

    for l in 0..4 {
        ret[l] = data[l + offset] as char;
    }
    ret
}

fn get_crc(data: &Vec<u8>, offset: usize) -> [u8; 4] {
    let mut ret = [0; 4];

    for l in 0..4 {
        ret[l] = data[l + offset];
    }
    ret
}

fn get_ihdr(data: &Vec<u8>, offset: usize) -> (IHDR, usize) {
    let mut byte_offset = offset;

    loop {
        let length = byte_to_u32(&data, byte_offset);
        byte_offset += 4;

        let chunk_type = get_chunk_type(&data, byte_offset);
        byte_offset += 4;

        if verify_chunk_type(&chunk_type, TYPE_IHDR) {
            let image_width = byte_to_u32(&data, byte_offset);
            byte_offset += 4;
            let image_height = byte_to_u32(&data, byte_offset);
            byte_offset += 4;
            let bit_depth = data[byte_offset];
            byte_offset += 1;
            let color_type = data[byte_offset];
            byte_offset += 1;
            let compress_method = data[byte_offset];
            byte_offset += 1;
            let filter_method = data[byte_offset];
            byte_offset += 1;
            let interlace_method = data[byte_offset];
            byte_offset += 1;
            // let crc = get_crc(&data, byte_offset);
            return (
                IHDR {
                    length,
                    chunk_type,
                    image_width,
                    image_height,
                    bit_depth,
                    color_type,
                    compress_method,
                    filter_method,
                    interlace_method,
                    // crc,
                },
                byte_offset + 4,
            );
        } else {
            byte_offset += length as usize + 4;
        }
    }
}

fn get_plte(data: &Vec<u8>, offset: usize) -> (PLTE, usize) {
    let mut byte_offset = offset;

    loop {
        let length = byte_to_u32(&data, byte_offset);
        byte_offset += 4;
        let chunk_type = get_chunk_type(&data, byte_offset);
        byte_offset += 4;
        let mut chunk_data = Vec::new();

        if verify_chunk_type(&chunk_type, TYPE_PLTE) {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for l in 0..length {
                match length % 3 {
                    0 => red = data[byte_offset + l as usize],
                    1 => green = data[byte_offset + l as usize],
                    2 => {
                        blue = data[byte_offset + l as usize];
                        chunk_data.push(RGB { red, green, blue });
                    }
                    _ => println!("Err!"),
                }
            }
            byte_offset += length as usize;
            // let crc = get_crc(&data, byte_offset);
            return (
                PLTE {
                    length,
                    chunk_type,
                    chunk_data,
                    // crc,
                },
                byte_offset + 4,
            );
        } else {
            byte_offset += length as usize + 4;
        }
    }
}

fn get_trns(data: &Vec<u8>, offset: usize) -> (TRNS, usize) {
    let mut byte_offset = offset;

    loop {
        let length = byte_to_u32(&data, byte_offset);
        byte_offset += 4;
        let chunk_type = get_chunk_type(&data, byte_offset);
        byte_offset += 4;
        let mut chunk_data = Vec::new();

        if verify_chunk_type(&chunk_type, TYPE_TRNS) {
            for l in 0..length {
                chunk_data.push(data[byte_offset + l as usize]);
            }
            byte_offset += length as usize + 4;
            // let crc = get_crc(&data, byte_offset);
            return (
                TRNS {
                    length,
                    chunk_type,
                    chunk_data,
                    // crc,
                },
                byte_offset + 4,
            );
        } else {
            byte_offset += length as usize + 4;
        }
    }
}

fn get_idat(data: &Vec<u8>, offset: usize) -> (IDAT, usize) {
    let mut byte_offset = offset;

    let mut chunk_data = Vec::new();
    let mut length = 0;

    loop {
        length += byte_to_u32(&data, byte_offset);
        byte_offset += 4;
        let chunk_type = get_chunk_type(&data, byte_offset);
        byte_offset += 4;

        if verify_chunk_type(&chunk_type, TYPE_IDAT) {
            for l in 0..length {
                chunk_data.push(data[l as usize]);
            }
            byte_offset += length as usize;
            // let crc = get_crc(&data, byte_offset);
            byte_offset += 4;
        } else if verify_chunk_type(&chunk_type, TYPE_IEND) {
            return (
                IDAT {
                    length,
                    chunk_type: *TYPE_IDAT,
                    chunk_data,
                    // crc,
                },
                byte_offset + 4,
            );
        } else {
            byte_offset += length as usize + 4;
        }
    }
}

fn get_iend(data: &Vec<u8>, offset: usize) -> (IEND, usize) {
    let mut byte_offset = offset;

    loop {
        let length = byte_to_u32(&data, byte_offset);
        byte_offset += 4;
        let chunk_type = get_chunk_type(&data, byte_offset);
        byte_offset += 4;

        if verify_chunk_type(&chunk_type, TYPE_IEND) {
            // let crc = get_crc(&data, byte_offset);
            return (
                IEND {
                    length,
                    chunk_type,
                    // crc,
                },
                byte_offset + 4,
            );
        } else {
            byte_offset += length as usize + 4;
        }
    }
}

pub fn get_png_data(data: Vec<u8>) -> PNG {
    let mut byte_offset = 8;

    let ret = get_ihdr(&data, byte_offset);
    let image_header = ret.0;
    byte_offset += ret.1;

    let ret = get_plte(&data, byte_offset);
    let palette = ret.0;
    byte_offset += ret.1;

    let ret = get_idat(&data, byte_offset);
    let image_data = ret.0;
    byte_offset += ret.1;

    let _ret = get_iend(&data, byte_offset);

    PNG {
        image_header,
        palette,
        image_data,
    }
}

fn get_usized_data(file_data: PNG) -> Vec<Vec<usize>> {
    // Encoded data line vector -> decoded pixel line vector
    let bit_depth = file_data.image_header.bit_depth;
    let color_type = file_data.image_header.color_type;
    let image_width = convert_image_dimension(file_data.image_header.image_width, color_type);

    let pixel_data = inflate_bytes(&file_data.image_data.chunk_data).unwrap();

    match bit_depth {
        1 | 2 | 4 => conv_from_line(ext_bit(pixel_data, bit_depth), image_width),
        8 => conv_from_line(convert_vec_type(pixel_data), image_width),
        16 => conv_from_line(join_byte(pixel_data), image_width),
        _ => vec![vec![0]],
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

// 画像の縦横（画素）を縦横（バイト）に変換する
fn convert_image_dimension(width: u32, color_type: u8) -> usize {
    match color_type {
        0 => (width + 1) as usize,
        2 => (width * 3 + 1) as usize,
        3 => (width + 1) as usize,
        4 => (width * 2 + 1) as usize,
        6 => (width * 4 + 1) as usize,
        _ => 0,
    }
}

// フィルター外したあとにやる
fn set_palette(pixel_data: Vec<Vec<usize>>, palette: Vec<RGB>) -> Vec<Vec<usize>> {
    let mut ret = Vec::new();

    for y in pixel_data {
        let mut ret_x = Vec::new();
        for x in y {
            ret_x.push(palette[x].red as usize);
            ret_x.push(palette[x].green as usize);
            ret_x.push(palette[x].blue as usize);
        }
        ret.push(ret_x);
    }
    ret
}

fn unfilter(pixel_data: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut ret = Vec::new();

    let y_max = pixel_data.len();
    let x_max = pixel_data[0].len();

    for y in 0..y_max {
        let mut ret_x = Vec::new();
        let filter_method = pixel_data[y][0];

        match filter_method {
            0 => {
                for x in 1..x_max {
                    ret_x.push(pixel_data[y][x]);
                }
            }
            1 => {
                ret_x.push(pixel_data[y][1]);

                for x in 2..x_max {
                    ret_x.push(pixel_data[y][x] + pixel_data[y][x - 1]);
                }
            }
            2 => {
                if y == 0 {
                    for x in 1..x_max {
                        ret_x.push(pixel_data[y][x]);
                    }
                } else {
                    for x in 1..x_max {
                        ret_x.push(pixel_data[y][x] + pixel_data[y - 1][x]);
                    }
                }
            }
            3 => {
                if y == 0 {
                    ret_x.push(pixel_data[y][1]);
                    for x in 2..x_max {
                        ret_x.push(pixel_data[y][x] + pixel_data[y][x - 1] / 2);
                    }
                } else {
                    ret_x.push(pixel_data[y][1] + pixel_data[y - 1][1] / 2);
                    for x in 2..x_max {
                        ret_x.push(
                            pixel_data[y][x] + (pixel_data[y - 1][x] + pixel_data[y][x - 1]) / 2,
                        );
                    }
                }
            }
            4 => {
                if y == 0 {
                    ret_x.push(paeth_predictor(pixel_data[y][1], 0, 0, 0));
                    for x in 2..x_max {
                        ret_x.push(paeth_predictor(
                            pixel_data[y][x],
                            pixel_data[y][x - 1],
                            0,
                            0,
                        ));
                    }
                } else {
                    ret_x.push(paeth_predictor(
                        pixel_data[y][1],
                        0,
                        pixel_data[y - 1][1],
                        0,
                    ));
                    for x in 2..x_max {
                        ret_x.push(paeth_predictor(
                            pixel_data[y][x],
                            pixel_data[y][x - 1],
                            pixel_data[y - 1][x],
                            pixel_data[y - 1][x - 1],
                        ));
                    }
                }
            }
            _ => ret_x.push(0),
        }
        ret.push(ret_x);
    }
    ret
}

// a: left
// b: upper
// c: left upper
fn paeth_predictor(x: usize, a: usize, b: usize, c: usize) -> usize {
    let p = a + b - c;
    let pa = p.abs_diff(a);
    let pb = p.abs_diff(b);
    let pc = p.abs_diff(c);

    if pa <= pb && pa <= pc {
        x + a
    } else if pb <= pc {
        x + b
    } else {
        x + c
    }
}

// PNGをバイト配列で読み込む
// 画素部分のデータを取り出す
// データをinflateする
// データを2次元vectorにする
// unfilterする
