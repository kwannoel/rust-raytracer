// To run this: `cargo run > image.ppm` from project root

mod encoder;

const IMAGE_PIXEL_WIDTH : i32 = 256;
const IMAGE_PIXEL_HEIGHT : i32 = 256;
const MAX_COLOUR_VALUE : i32 = 255;

fn main() {

    encoder::ppm_headers(IMAGE_PIXEL_WIDTH, IMAGE_PIXEL_HEIGHT, MAX_COLOUR_VALUE);

    // Write the pixels from top to bottom row
    for height in (0..IMAGE_PIXEL_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", height);
        // Write the pixels for each row from left to right
        for width in 0..IMAGE_PIXEL_WIDTH {
            let r : f64 = f64::from(width) / f64::from(IMAGE_PIXEL_WIDTH - 1);
            let g : f64 = f64::from(height) / f64::from(IMAGE_PIXEL_HEIGHT - 1);
            let b : f64 = 0.25;

            let ir : i32 = (r * 255.999) as i32;
            let ig : i32 = (g * 255.999) as i32;
            let ib : i32 = (b * 255.999) as i32;

            println!("{} {} {}", ir, ig, ib)
        }
    }
    eprintln!("\nDone.\n")
}
