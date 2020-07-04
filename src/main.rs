fn main() {
    const IMAGE_WIDTH : i32 = 256;
    const IMAGE_HEIGHT : i32 = 256;
    const MAX_COLOUR_VALUE : i32 = 255;

    // PPM specifications are here: http://davis.lbl.gov/Manuals/NETPBM/doc/ppm.html
    // PPM example: https://en.wikipedia.org/wiki/Netpbm#PPM_example

    // Write "P3" header for PPM format
    println!("P3");

    // Write image width, height
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);

    // Write maximum color value
    println!("{}", MAX_COLOUR_VALUE);

    // Write the pixels from top to bottom row
    for height in (0..IMAGE_HEIGHT).rev() {
        // Write the pixels for each row from left to right
        for width in 0..IMAGE_WIDTH {
            let r : f64 = f64::from(width) / f64::from(IMAGE_WIDTH - 1);
            let g : f64 = f64::from(height) / f64::from(IMAGE_HEIGHT - 1);
            let b : f64 = 0.25;

            let ir : i32 = (r * 255.999) as i32;
            let ig : i32 = (g * 255.999) as i32;
            let ib : i32 = (b * 255.999) as i32;

            println!("{} {} {}", ir, ig, ib)
        }
    }
    eprintln!("done")
}
