use inflate::inflate_bytes;
use std::fs::File;
use std::io::Read;

use crate::binarization::conv_from_line;

const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]; // PNGファイルの証

// Chunk Typeの宣言部
const TYPE_IHDR: &[u8; 4] = &[73, 72, 68, 82];
const TYPE_PLTE: &[u8; 4] = &[80, 76, 84, 69];
const TYPE_TRNS: &[u8; 4] = &[116, 82, 78, 83];
const TYPE_IDAT: &[u8; 4] = &[73, 68, 65, 84];
const TYPE_IEND: &[u8; 4] = &[73, 69, 78, 68];

// RGBを一つで扱うための構造体
struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

// イメージヘッダの情報を格納する構造体
pub struct IHDR {
    length: u32,
    chunk_type: [u8; 4],
    image_width: u32,
    image_height: u32,
    bit_depth: u8,
    color_type: u8,
    compress_method: u8,
    filter_method: u8,
    interlace_method: u8,
    // crc: [u8; 4],
}

// パレットの情報を格納する構造体
pub struct PLTE {
    length: u32,
    chunk_type: [u8; 4],
    chunk_data: Vec<RGB>,
    // crc: [u8; 4],
}

// tRNSチャンク（透明度）の情報を格納する構造体．使うかもしれないけど使わないかもしれない．
struct TRNS {
    length: u32,
    chunk_type: [u8; 4],
    chunk_data: Vec<u8>,
    // crc: [u8; 4],
}

// 画像の画素データの情報を格納する構造体
pub struct IDAT {
    length: usize,
    chunk_type: [u8; 4],
    pub chunk_data: Vec<u8>,
    // crc: [u8; 4],
}

// 画像ファイルの終端を示すデータチャンク．使うのかこれ
struct IEND {
    length: u32,
    chunk_type: [u8; 4],
    // crc: [u8; 4],
}

// Chunk Typeを確認する
fn verify_chunk_type(chunk_type: &[u8; 4], type_ex: &[u8; 4]) -> bool {
    let mut ret = false;
    for i in 0..4 {
        ret |= chunk_type[i] == type_ex[i];
    }
    ret
}

// ファイルを読み込んでu8のvectorに変換するやつ
fn file_to_vec(filename: &str) -> Vec<u8> {
    let mut file: File = File::open(filename).expect("Failed to open file");

    let mut data: Vec<u8> = Vec::new();

    file.read_to_end(&mut data).expect("Failed to read data");

    data
}

// 読み込んだファイルのPNG Signatureを照合する関数
fn is_file_png(data: &Vec<u8>, offset: usize) -> (bool, usize) {
    let mut is_png = false;
    let mut offset = offset;

    for i in 0..8 {
        is_png &= data[i] == PNG_SIGNATURE[i];
        offset += 1;
    }
    (is_png, offset)
}

// u8のVecの4バイト分を1つの数値に変換するやつ
fn byte_to_u32(data: &Vec<u8>, offset: usize) -> (u32, usize) {
    let mut ret: u32 = 0;

    for i in 0..4 {
        ret += (data[i + offset] as u32) << ((4 - i - 1) * 8);
    }
    // println!("ret={}", ret);
    (ret, offset + 4)
}

// Chunk Typeを取り出す
fn get_chunk_type(data: &Vec<u8>, offset: usize) -> ([u8; 4], usize) {
    let mut ret: [u8; 4] = [0; 4];

    for i in 0..4 {
        ret[i] = data[i + offset];
    }
    (ret, offset + 4)
}

// CRCを取り出す
fn get_crc(data: &Vec<u8>, offset: usize) -> ([u8; 4], usize) {
    let mut ret: [u8; 4] = [0; 4];

    for i in 0..4 {
        ret[i] = data[i + offset];
    }
    println!("Get CRC");
    (ret, offset + 4)
}

// IHDRを取り出す
fn get_ihdr(data: &Vec<u8>, offset: usize) -> Result<(IHDR, usize), String> {
    println!("Get IHDR");

    let (length, offset) = byte_to_u32(data, offset);
    let (chunk_type, offset) = get_chunk_type(data, offset);

    println!("offset = {}", offset);
    if verify_chunk_type(&chunk_type, TYPE_IHDR) {
        let (image_width, offset) = byte_to_u32(data, offset);

        let (image_height, mut offset) = byte_to_u32(data, offset);

        let bit_depth = data[offset];
        offset += 1;
        let color_type = data[offset];
        offset += 1;
        let compress_method = data[offset];
        offset += 1;
        let filter_method = data[offset];
        offset += 1;
        let interlace_method = data[offset];
        offset += 1;
        let (_crc, offset) = get_crc(data, offset);

        Ok((
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
                // _crc,
            },
            offset,
        ))
    } else {
        Err(String::from("IHDR is not located head of file"))
    }
}

// PLTEを取り出す
fn get_plte(data: &Vec<u8>, offset: usize) -> Result<(PLTE, usize), String> {
    println!("Get PLTE");

    loop {
        let (length, offset) = byte_to_u32(data, offset);

        let (chunk_type, mut offset) = get_chunk_type(data, offset);

        let mut chunk_data: Vec<RGB> = Vec::new();

        if verify_chunk_type(&chunk_type, TYPE_PLTE) {
            let mut r = 0;
            let mut g = 0;

            for i in 0..length {
                match i % 3 {
                    0 => r = data[offset + i as usize],
                    1 => g = data[offset + i as usize],
                    2 => chunk_data.push(RGB {
                        red: r,
                        green: g,
                        blue: data[offset + i as usize],
                    }),
                    _ => return Err(String::from("Mod error")),
                }
            }
            let (_crc, offset) = get_crc(data, offset);

            return Ok((
                PLTE {
                    length,
                    chunk_type,
                    chunk_data,
                    // _crc,
                },
                offset,
            ));
        } else if verify_chunk_type(&chunk_type, TYPE_IEND) {
            return Err(String::from("PLTE chunk not found"));
        } else {
            offset += length as usize + 4;
        }
    }
}

// tRNSを取り出す
fn get_trns(data: &Vec<u8>, offset: usize) -> Result<(TRNS, usize), String> {
    println!("Get tRNS");

    loop {
        let (length, offset) = byte_to_u32(data, offset);

        let (chunk_type, mut offset) = get_chunk_type(data, offset);

        let mut chunk_data: Vec<u8> = Vec::new();

        if verify_chunk_type(&chunk_type, TYPE_TRNS) {
            for i in 0..length {
                chunk_data.push(data[offset + i as usize]);
                offset += 1;
            }
            let (_crc, offset) = get_crc(data, offset);

            return Ok((
                TRNS {
                    length,
                    chunk_type,
                    chunk_data,
                    // _crc,
                },
                offset,
            ));
        } else if verify_chunk_type(&chunk_type, TYPE_IEND) {
            return Err(String::from("tRNS chunk is not found"));
        } else {
            offset += length as usize + 4;
        }
    }
}

// IDATを取り出す
fn get_idat(data: &Vec<u8>, offset: usize) -> Result<(IDAT, usize), String> {
    println!("Get IDAT");

    let mut chunk_data = Vec::new();
    let mut length = 0;

    loop {
        let ret = byte_to_u32(data, offset);
        length += ret.0 as usize;
        println!("length1 = {}", length);
        let offset = ret.1;
        let (chunk_type, mut offset) = get_chunk_type(data, offset);
        // print_ary(&chunk_type);

        if verify_chunk_type(&chunk_type, TYPE_IDAT) {
            println!("Match IDAT");
            for i in 0..length {
                chunk_data.push(data[offset + i]);
                offset += 1;
            }
            let (_crc, offset) = get_crc(data, offset);
        } else if verify_chunk_type(&chunk_type, TYPE_IEND) {
            println!("Match IEND");
            return Ok((
                IDAT {
                    length,
                    chunk_type,
                    chunk_data,
                    // _crc,
                },
                offset,
            ));
        } else {
            // println!("else");
            println!("length2 = {}", length);
            offset += length as usize + 4;
            length = 0;
            // println!("ofs = {}", offset);
        }
    }
}

fn ext_bit(pixel_data: Vec<u8>, bit_depth: u8) -> Result<Vec<usize>, String> {
    let mut ret: Vec<usize> = Vec::new();

    match bit_depth {
        1 | 2 | 4 => {
            let mut mask: u8;
            let init_mask = gen_mask(bit_depth);

            for d in pixel_data {
                mask = init_mask;
                for k in 0..(8 / bit_depth) {
                    ret.push(((d & mask) >> ((8 / bit_depth - k - 1) * bit_depth)) as usize);
                    mask = mask >> bit_depth;
                }
            }
            Ok(ret)
        }
        8 => {
            for d in pixel_data {
                ret.push(d as usize);
            }
            Ok(ret)
        }
        16 => {
            let mut upper_digit: u16 = 0;
            for l in 0..pixel_data.len() {
                match l % 2 {
                    0 => upper_digit = pixel_data[l] as u16,
                    1 => ret.push(((upper_digit << 8) | pixel_data[l] as u16) as usize),
                    _ => return Err(String::from("Mod err")),
                }
            }
            Ok(ret)
        }
        _ => return Err(String::from("Unknown bit depth")),
    }
}

fn gen_mask(bit_depth: u8) -> u8 {
    let mut mask: u8 = 0x80;

    for _i in 0..bit_depth {
        mask |= mask >> 1;
    }
    mask
}

// 画像の縦横（画素）を縦横（バイト）に変換する
fn get_image_byte_dimention(image_width: u32, color_type: u8) -> Result<usize, String> {
    match color_type {
        0 | 3 => Ok((image_width + 1) as usize),
        2 => Ok((image_width * 3 + 1) as usize),
        4 => Ok((image_width * 2 + 1) as usize),
        6 => Ok((image_width * 4 + 1) as usize),
        _ => Err(String::from("Unknown color type")),
    }
}

// フィルターを外す
pub fn unfilter(pixel_data: Vec<Vec<usize>>, modifier: usize) -> Result<Vec<Vec<usize>>, String> {
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
            _ => return Err(String::from("Unknown filter method")),
        }
        ret.push(ret_x);
    }
    for y in 0..pixel_data.len() {
        for x in 0..pixel_data[0].len() {
            ret[y][x] %= modifier;
        }
    }
    Ok(ret)
}

// 変数名は公式の仕様書と同じですので文句は受け付けません
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

pub fn get_pixel_data(filename: &str) -> Result<Vec<Vec<usize>>, String> {
    let data = file_to_vec(filename);

    let (v, offset) = is_file_png(&data, 0);
    if v {
        return Err(String::from("File is not PNG"));
    }

    println!("gpd offset = {}", offset);

    let (ihdr, offset) = get_ihdr(&data, offset).unwrap();

    println!("gpd offset2 = {}", offset);

    let image_width = ihdr.image_width;

    println!("image width = {}", image_width);

    let bit_depth = ihdr.bit_depth;
    let color_type = ihdr.color_type;

    if color_type == 3 {
        let (plte, offset) = get_plte(&data, offset).unwrap();

        let (idat, offset) = get_idat(&data, offset).unwrap();

        let pixel_data = set_palette(
            unfilter(
                conv_from_line(
                    ext_bit(inflate_bytes(&idat.chunk_data).unwrap(), bit_depth).unwrap(),
                    image_width as usize,
                ),
                bit_depth as usize,
            )
            .unwrap(),
            plte.chunk_data,
        );

        Ok(pixel_data)
    } else {
        let (idat, offset) = get_idat(&data, offset).unwrap();

        let pixel_data = unfilter(
            conv_from_line(
                ext_bit(inflate_bytes(&idat.chunk_data).unwrap(), bit_depth).unwrap(),
                image_width as usize,
            ),
            bit_depth as usize,
        )
        .unwrap();

        Ok(pixel_data)
    }
}

fn print_ary(ary: &[u8; 4]) {
    print!("[");

    for i in 0..4 {
        print!("{}, ", ary[i]);
    }
    println!("]");
}
