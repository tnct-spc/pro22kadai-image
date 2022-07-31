use std::fs::File;
use std::io::Read;

const ihdr: &[char; 4] = &['I', 'H', 'D', 'R'];
const plte: &[char; 4] = &['P', 'L', 'T', 'E'];
const idat: &[char; 4] = &['I', 'D', 'A', 'T'];
const iend: &[char; 4] = &['I', 'E', 'N', 'D'];

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
    crc: [u8; 4],
}

struct PNG {
    image_header: IHDR,
    palette: PLTE,
    image_data: Vec<IDAT>,
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

// Open file and convert to Vec<u8>
fn file_to_vec(filename: String) -> Vec<u8> {
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
        ret += (data[data_count - l + offset] as usize * 256 ^ (l)) as u32;
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

        if verify_chunk_type(&chunk_type, ihdr) {
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
            let crc = get_crc(&data, byte_offset);
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
                    crc,
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

        if verify_chunk_type(&chunk_type, plte) {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for l in 0..length {
                if length % 3 == 0 {
                    red = data[byte_offset + l as usize];
                } else if length % 3 == 1 {
                    green = data[byte_offset + l as usize];
                } else if length % 3 == 2 {
                    blue = data[byte_offset + l as usize];
                    chunk_data.push(RGB { red, green, blue });
                }
            }
            byte_offset += length as usize;
            let crc = get_crc(&data, byte_offset);
            return (
                PLTE {
                    length,
                    chunk_type,
                    chunk_data,
                    crc,
                },
                byte_offset + 4,
            );
        } else {
            byte_offset += length as usize + 4;
        }
    }
}

fn get_idat(data: &Vec<u8>, offset: usize) -> (Vec<IDAT>, usize) {
    let mut byte_offset = offset;
    let mut ret = Vec::new();

    loop {
        let length = byte_to_u32(&data, byte_offset);
        byte_offset += 4;
        let chunk_type = get_chunk_type(&data, byte_offset);
        byte_offset += 4;
        let mut chunk_data = Vec::new();

        if verify_chunk_type(&chunk_type, idat) {
            for l in 0..length {
                chunk_data.push(data[l as usize]);
            }
            byte_offset += length as usize;
            let crc = get_crc(&data, byte_offset);
            byte_offset += 4;

            ret.push(IDAT {
                length,
                chunk_type,
                chunk_data,
                crc,
            });
        } else if verify_chunk_type(&chunk_type, iend) {
            return (ret, byte_offset + 4);
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

        if verify_chunk_type(&chunk_type, iend) {
            let crc = get_crc(&data, byte_offset);
            return (
                IEND {
                    length,
                    chunk_type,
                    crc,
                },
                byte_offset + 4,
            );
        } else {
            byte_offset += length as usize + 4;
        }
    }
}

fn get_png_data(data: &Vec<u8>) -> PNG {
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

    let ret = get_iend(&data, byte_offset);
    let image_end = ret.0;
    byte_offset += ret.1;

    PNG {
        image_header,
        palette,
        image_data,
    }
}

fn get_pixel_data<T>(png: &PNG) -> T {
    let color_type = png.image_header.color_type;
    let palette_length = png.palette.length;
    let data_length = png.image_header.length;

    //    let mut ret = Vec::new();

    if color_type == 0 {
        // image is greyscale
    } else if color_type == 2 {
        // image is truecolor
    } else if color_type == 3 {
        // image is indexcolor, required palette
    } else if color_type == 4 {
        // image is greyscale + alpha
    } else if color_type == 6 {
        // image is truecolor + alpha
    }
}