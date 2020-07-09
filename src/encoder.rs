// PPM specifications are here: http://davis.lbl.gov/Manuals/NETPBM/doc/ppm.html
// PPM example: https://en.wikipedia.org/wiki/Netpbm#PPM_example

// Encodes ppm headers
pub fn ppm_headers(image_pixel_width: i32, image_pixel_height: i32, max_colour_value: i32) {
    // Write "P3" header for PPM format
    println!("P3");

    // Write image width, height
    println!("{} {}", image_pixel_width, image_pixel_height);

    // Write maximum color value
    println!("{}", max_colour_value);
}
