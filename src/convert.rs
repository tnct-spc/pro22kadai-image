use std::fs::File;

struct rgb {
    red: u8,
    green: u8,
    blue: u8,
}

struct ihdr {
    length: Vec<u8>,
    chunk_type: Vec<u8>,
    image_width: Vec<u8>,
    image_height: Vec<u8>,
    bit_depth: u8,
    color_type: u8,
    compress_method: u8,
    filter_method: u8,
    interlace_method: u8,
    cyclic_redundancy_check: Vec<u8>,
}

struct plte {
    length: usize,
    chunk_type: Vec<u8>,
    chunk_data: Vec<rgb>,
    cyclic_redundancy_check: Vec<u8>,
}

struct idat {
    length: Vec<u8>,
    chunk_type: Vec<u8>,
    chunk_data: Vec<u8>,
    cyclic_redundancy_check: Vec<u8>,
}

struct iend {
    length: Vec<u8>,
    chunk_type: Vec<u8>,
    cyclic_redundancy_check: Vec<u8>,
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

fn slice(data: &Vec<_>, offset: usize, size: usize) -> Vec<_> {
    let mut ret = Vec::new();

    for i in 0..size {
        ret.push(data[i + offset]);
    }
    ret
}

fn byte_vec_to_usize(data: &Vec<u8>) -> usize {
    let l_max = data.len();
    let ret: usize = 0;

    for l in 0..l_max {
        ret += data[l] * 256 ^ l;
    }
    ret
}

fn get_image_header(data: &Vec<u8>) -> ihdr {
    // 各ヘッダのオフセット + PNG Signatureのオフセット8バイト
    ihdr {
        length: slice(&data, 0x0000 + 8, 4),
        chunk_type: slice(&data, 0x0004 + 8, 4),
        image_width: slice(&data, 0x0008 + 8, 4),
        image_height: slice(&data, 0x000C + 8, 4),
        bit_depth: data[0x0010],
        color_type: data[0x0011],
        compress_method: data[0x0012],
        filter_method: data[0x0013],
        interlace_method: data[0x0014],
        cyclic_redundancy_check: slice(&data, 0x0015 + 8, 4),
    }
}

fn get_palette(data: &Vec<u8>) -> plte {
    let length_vec = slice(&data, 0x0000 + 8 + 25, 4);
    let length = byte_vec_to_usize(&data);

    let chunk_type = slice(&data, 0x0004 + 8 + 25, 4);

    let mut palette_data: rgb;
    let mut chunk_data: Vec<rgb> = Vec::new();
    let mut color_flag = 0;

    for d in 0..length {
        if color_flag == 0 {
            palette_data.red = data[i + 8 + 25 + 0x0008];
        } else if color_flag == 1 {
            palette_data.green = data[i + 8 + 25 + 0x0008];
        } else {
            palette_data.blue = data[i + 8 + 25 + 0x0008];
            chunk_data.push(palette_data);
        }
        color_flag = d % 3;
    }
    plte {
        length: length,
        chunk_type: chunk_type,
        chunk_data: chunk_data,
        cyclic_redundancy_check: slice(&data, 8 + length + 8 + 25, 4),
    }
}

fn get_color_vec(data: &Vec<u8>) {
    // get Image Header
    let image_header = get_image_header(&data);

    // get Palette
    if image_header.color_type == 3 {
        let plt_byte_offset = 18;
    } else {
        let plt_byte_offset = 0;
    }
}
