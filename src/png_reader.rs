use inflate::inflate_bytes;
use std::fs::File;
use std::io::Read;

use crate::binarization::conv_from_line;

const PNG_SIGNATURE: [u8; 8] = [0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]; // PNGファイルの証

// Chunk Typeの宣言部
const TYPE_IHDR: &[char; 4] = &['I', 'H', 'D', 'R'];
const TYPE_PLTE: &[char; 4] = &['P', 'L', 'T', 'E'];
const TYPE_TRNS: &[char; 4] = &['t', 'R', 'N', 'S'];
const TYPE_IDAT: &[char; 4] = &['I', 'D', 'A', 'T'];
const TYPE_IEND: &[char; 4] = &['I', 'E', 'N', 'D'];

pub enum PNGReadErr {
    FileIsNotPNG,
}

// RGBを一つで扱うための構造体
struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

// イメージヘッダの情報を格納する構造体
pub struct IHDR {
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

// パレットの情報を格納する構造体
pub struct PLTE {
    length: u32,
    chunk_type: [char; 4],
    chunk_data: Vec<RGB>,
    // crc: [u8; 4],
}

// tRNSチャンク（透明度）の情報を格納する構造体．使うかもしれないけど使わないかもしれない．
struct TRNS {
    length: u32,
    chunk_type: [char; 4],
    chunk_data: Vec<u8>,
    // crc: [u8; 4],
}

// 画像の画素データの情報を格納する構造体
pub struct IDAT {
    length: u32,
    chunk_type: [char; 4],
    pub chunk_data: Vec<u8>,
    // crc: [u8; 4],
}

// 画像ファイルの終端を示すデータチャンク．使うのかこれ
struct IEND {
    length: u32,
    chunk_type: [char; 4],
    // crc: [u8; 4],
}

// ファイルを読み込んでu8のvectorに変換するやつ
fn file_to_vec(filename: &str) -> Vec<u8> {
    let mut file: File = File::open(filename).expect("Failed to open file");

    let mut data: Vec<u8> = Vec::new();

    file.read_to_end(&mut data).expect("Failed to read data");

    data
}

// 読み込んだファイルのPNG Signatureを照合する関数
fn is_file_png(data: &Vec<u8>, offset: &mut usize) -> bool {
    let mut is_png = false;

    for i in 0..8 {
        is_png &= data[i] == PNG_SIGNATURE[i];
        *offset += 1;
    }
    is_png
}

// ファイルのvectorでイメージヘッダによくある4バイト分で1つの数を表すやつを数値に変換する
fn byte_to_u32(data: &Vec<u8>, offset: &mut usize) -> u32 {
    let data_count = data.len();
    let mut ret: u32 = 0;

    for l in 4..0 {
        ret += (data[data_count - l + *offset] << (l * 8)) as u32;
        *offset += 1;
    }
    ret
}

// ファイルのvectorからChunk Typeを取り出す関数
fn get_chunk_type(data: &Vec<u8>, offset: &mut usize) -> [char; 4] {
    let mut ret: [char; 4] = ['\0'; 4];

    for i in 0..4 {
        ret[i] = data[i + *offset] as char;
        *offset += 1;
    }
    ret
}

// CRC（チェックデジット的なあれ）を取得する
fn get_crc(data: &Vec<u8>, offset: &mut usize) -> [u8; 4] {
    let mut ret = [0; 4];

    for i in 0..4 {
        ret[i] = data[i + *offset];
        *offset += 1;
    }
    ret
}

fn get_ihdr(data: &Vec<u8>, offset: &mut usize) -> Result<IHDR, &'static str> {
    println!("Get IHDR");

    let length: u32 = byte_to_u32(data, offset);
    let chunk_type = get_chunk_type(data, offset);

    if &chunk_type == TYPE_IHDR {
        let image_width: u32 = byte_to_u32(data, offset);
        let image_height: u32 = byte_to_u32(data, offset);
        let bit_depth: u8 = data[*offset];
        *offset += 1;
        let color_type: u8 = data[*offset];
        *offset += 1;
        let compress_method: u8 = data[*offset];
        *offset += 1;
        let filter_method: u8 = data[*offset];
        *offset += 1;
        let interlace_method: u8 = data[*offset];
        *offset += 1;
        let _crc = get_crc(data, offset);

        Ok(IHDR {
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
        })
    } else {
        Err("IHDR is not located head of file")
    }
}

fn get_plte(data: &Vec<u8>, offset: &mut usize) -> Result<PLTE, &'static str> {
    println!("Get PLTE");

    loop {
        let length: u32 = byte_to_u32(data, offset);
        let chunk_type: [char; 4] = get_chunk_type(data, offset);
        let mut chunk_data: Vec<RGB> = Vec::new();

        match &chunk_type {
            TYPE_PLTE => {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;

                for l in 0..length {
                    match l % 3 {
                        0 => red = data[*offset + l as usize],
                        1 => green = data[*offset + l as usize],
                        2 => {
                            blue = data[*offset + l as usize];
                            chunk_data.push(RGB { red, green, blue });
                        }
                        _ => return Err("Failed to get Mod"),
                    }
                    *offset += 1;
                }
                let crc = get_crc(data, offset);
                return Ok(PLTE {
                    length,
                    chunk_type,
                    chunk_data,
                    // crc,
                });
            }
            TYPE_IEND => return Err("PLTE chunk not found"),
            _ => *offset += length as usize + 4,
        }
    }
}

fn get_trns(data: &Vec<u8>, offset: &mut usize) -> Result<TRNS, &'static str> {
    println!("Get tRNS");

    loop {
        let length = byte_to_u32(data, offset);
        let chunk_type = get_chunk_type(data, offset);
        let mut chunk_data: Vec<u8> = Vec::new();

        match &chunk_type {
            TYPE_TRNS => {
                for l in 0..length {
                    chunk_data.push(data[*offset + l as usize]);
                    *offset += 1;
                }
                let crc = get_crc(&data, offset);

                return Ok(TRNS {
                    length,
                    chunk_type,
                    chunk_data,
                    // crc,
                });
            }
            TYPE_IEND => return Err("tRNS chunk not found"),
            _ => *offset += length as usize + 4,
        }
    }
}

fn get_idat(data: &Vec<u8>, offset: &mut usize) -> Result<IDAT, &'static str> {
    println!("Get IDAT");

    let mut chunk_data = Vec::new();
    let mut length = 0;

    loop {
        length += byte_to_u32(data, offset);
        let chunk_type = get_chunk_type(data, offset);

        match &chunk_type {
            TYPE_IDAT => {
                for l in 0..length {
                    chunk_data.push(data[*offset + l as usize]);
                    *offset += 1;
                }
                let _crc = get_crc(data, offset);
            }
            TYPE_IEND => {
                return Ok(IDAT {
                    length,
                    chunk_type,
                    chunk_data,
                    // crc,
                });
            }
            _ => *offset += length as usize + 4,
        }
    }
}

pub fn get_pixel_data(filename: &str) -> Result<Vec<Vec<usize>>, &'static str> {
    let data: Vec<u8> = file_to_vec(filename);
    let mut offset: usize = 0;

    println!("offset = {}", offset);

    if is_file_png(&data, &mut offset) {
        return Err("This file is not PNG Image");
    }
    println!("offset = {}", offset);
    // まずIHDRチャンクを取得し，PLTEを取るかどうかを決める
    let ihdr = get_ihdr(&data, &mut offset).unwrap();

    let image_width = ihdr.image_width;
    let image_height = ihdr.image_height;
    let bit_depth = ihdr.bit_depth;
    let color_type = ihdr.color_type;

    let idat = get_idat(&data, &mut offset).unwrap();

    let pixel_data = unfilter(
        conv_from_line(
            ext_bit(inflate_bytes(&idat.chunk_data).unwrap(), bit_depth).unwrap(),
            image_width as usize,
        ),
        bit_depth as usize,
    )
    .unwrap();

    // color_typeが3だった場合はPLTEチャンクを取得し，各ピクセルをパレットに置換する
    if color_type == 3 {
        let plte = get_plte(&data, &mut offset).unwrap();

        let pixel_data = set_palette(pixel_data, plte.chunk_data);
        Ok(pixel_data)
    } else {
        Ok(pixel_data)
    }
}

fn ext_bit(pixel_data: Vec<u8>, bit_depth: u8) -> Result<Vec<usize>, &'static str> {
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
                    _ => return Err("Mod err"),
                }
            }
            Ok(ret)
        }
        _ => Err("Unknown bit depth"),
    }
}

fn gen_mask(digit: u8) -> u8 {
    let mut mask: u8 = 0x80;

    for _i in 0..digit {
        mask |= mask >> 1;
    }
    mask
}

// 画像の縦横（画素）を縦横（バイト）に変換する
fn get_image_byte_dimention(image_width: u32, color_type: u8) -> Result<usize, &'static str> {
    match color_type {
        0 | 3 => Ok((image_width + 1) as usize),
        2 => Ok((image_width * 3 + 1) as usize),
        4 => Ok((image_width * 2 + 1) as usize),
        6 => Ok((image_width * 4 + 1) as usize),
        _ => Err("Unknown color type"),
    }
}

// フィルターを外す
pub fn unfilter(
    pixel_data: Vec<Vec<usize>>,
    modifier: usize,
) -> Result<Vec<Vec<usize>>, &'static str> {
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
            _ => return Err("Unknown filter method"),
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
