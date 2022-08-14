use image::GenericImageView;

pub fn get_pixel_data(
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
