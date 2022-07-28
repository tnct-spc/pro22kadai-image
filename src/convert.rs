use std::fs::File;

struct GREYSCALE {
    bright: u8,
    alpha: u8,
}

struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

struct DATACHUNK {
    length: usize,
    chunk_type: [char; 4],
    chunk_data: Vec<u8>,
    cyclic_redundancy_check: [u8; 4],
}

struct IHDR {
    length: usize,
    chunk_type: [char; 4],
    image_width: usize,
    image_height: usize,
    bit_depth: usize,
    color_type: usize,
    compress_method: usize,
    filter_method: usize,
    interlace_method: usize,
    cyclic_redundancy_check: [u8; 4],
}

struct PLTE {
    length: usize,
    chunk_type: [char; 4],
    chunk_data: Vec<GREYSCALE>,
    cyclic_redundancy_check: [u8; 4],
}

struct IDAT {
    length: usize,
    chunk_type: [char; 4],
    chunk_data: Vec<u8>,
    cyclic_redundancy_check: [u8; 4],
}

struct IEND {
    length: usize,
    chunk_type: [char; 4],
    cyclic_redundancy_check: [u8; 4],
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

fn vec_to_ary(data: &Vec<u8>, offset: usize, size: usize) -> [u8; size] {
    let mut ret = [0; size];

    for i in 0..size {
        ret[i] = data[i + offset];
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

fn get_ihdr(data: &Vec<u8>) -> IHDR {}

fn get_image_vec(data: &Vec<u8>) -> Vec<u8> {}
