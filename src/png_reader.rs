use base64::{decode, encode};
use image::{GenericImageView, GrayImage, Luma, Pixel};
use std::fs::File;
use std::io::Read;
use std::mem::size_of;

pub fn get_color_data_from_filename(
    filename: &str,
) -> (
    Vec<Vec<usize>>,
    Vec<Vec<usize>>,
    Vec<Vec<usize>>,
    Vec<Vec<usize>>,
) {
    let img = image::open(filename).unwrap();

    let image_width = img.width();
    let image_height = img.height();

    let mut red_pixels = Vec::new();
    let mut green_pixels = Vec::new();
    let mut blue_pixels = Vec::new();
    let mut alpha_pixels = Vec::new();

    for y in 0..image_height {
        let mut red_line = Vec::new();
        let mut green_line = Vec::new();
        let mut blue_line = Vec::new();
        let mut alpha_line = Vec::new();

        for x in 0..image_width {
            let d = img.get_pixel(x, y);
            red_line.push(d[0] as usize);
            green_line.push(d[1] as usize);
            blue_line.push(d[2] as usize);
            alpha_line.push(d[3] as usize);
        }
        red_pixels.push(red_line);
        green_pixels.push(green_line);
        blue_pixels.push(blue_line);
        alpha_pixels.push(alpha_line);
    }
    (red_pixels, green_pixels, blue_pixels, alpha_pixels)
}

pub fn get_gray_data_from_filename(filename: &str) -> Vec<Vec<usize>> {
    let img = image::open(filename).unwrap().to_luma8();

    let image_width = img.width();
    let image_height = img.height();

    let mut image_data = Vec::new();

    for y in 0..image_height {
        let mut line_data = Vec::<usize>::new();

        for x in 0..image_width {
            let d = img.get_pixel(x, y);
            line_data.push(d[0] as usize);
        }
        image_data.push(line_data);
    }
    image_data
}

pub fn get_color_data_from_base64(
    filedata: String,
) -> (
    Vec<Vec<usize>>,
    Vec<Vec<usize>>,
    Vec<Vec<usize>>,
    Vec<Vec<usize>>,
) {
    let decoded_data = decode(filedata).unwrap();

    let img = image::load_from_memory(&decoded_data).unwrap();

    let image_width = img.width();
    let image_height = img.height();

    let mut red_pixels = Vec::new();
    let mut green_pixels = Vec::new();
    let mut blue_pixels = Vec::new();
    let mut alpha_pixels = Vec::new();

    for y in 0..image_height {
        let mut red_line = Vec::new();
        let mut green_line = Vec::new();
        let mut blue_line = Vec::new();
        let mut alpha_line = Vec::new();

        for x in 0..image_width {
            let d = img.get_pixel(x, y);
            red_line.push(d[0] as usize);
            green_line.push(d[1] as usize);
            blue_line.push(d[2] as usize);
            alpha_line.push(d[3] as usize);
        }
        red_pixels.push(red_line);
        green_pixels.push(green_line);
        blue_pixels.push(blue_line);
        alpha_pixels.push(alpha_line);
    }
    (red_pixels, green_pixels, blue_pixels, alpha_pixels)
}

pub fn get_gray_data_from_base64(filedata: String) -> Vec<Vec<usize>> {
    let decoded_data = decode(filedata).unwrap();

    let img = image::load_from_memory(&decoded_data).unwrap().to_luma8();

    let image_width = img.width();
    let image_height = img.height();

    let mut image_data = Vec::new();

    for y in 0..image_height {
        let mut line_data = Vec::<usize>::new();

        for x in 0..image_width {
            let d = img.get_pixel(x, y);
            line_data.push(d[0] as usize);
        }
        image_data.push(line_data);
    }
    image_data
}

pub fn png_to_base64(filename: &str) -> String {
    let mut file_data = Vec::new();
    let _f = File::open(filename).unwrap().read_to_end(&mut file_data);

    let encoded_data = encode(&file_data);

    encoded_data
}
