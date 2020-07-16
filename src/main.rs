// To run this: `cargo run > image.ppm` from project root

#[macro_use]
extern crate lazy_static;
extern crate rand;

mod color;
mod encoder;
mod point;
mod ray;
mod sphere;
mod vec3;
mod utils;
mod hittable;
mod world;
mod camera;
mod material;

use color::Color;
use point::Point;
use ray::Ray;
use vec3::Vec3;
use sphere::Sphere;
use world::World;
use camera::Camera;
use material::{Lambertian, Material, Metal};

// NOTE
// Convention for coordinates is such that towards the image from camera is -ve
// Toward the camera from the image is +ve
//                        |
// ORIGIN  ----(-ve)----> |
//         <---(+ve)----- |
//                        |
//                IMAGE
//

/* ==========
 * = CAMERA =
 * ==========
 */

const ORIGIN: Point = Point { x: 0.0, y: 0.0, z: 0.0 };

/* =========
 * = IMAGE =
 * =========
 */

// IMAGE DIMENSIONS in NUMBER OF PIXELS
const ASPECT_RATIO: f64 = 16.0 / 9.0; // width / height
const IMAGE_PIXEL_HEIGHT: i32 = 216;
const IMAGE_PIXEL_WIDTH: i32 = (IMAGE_PIXEL_HEIGHT as f64 * ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;

// IMAGE DIMENSIONS in CARTESIAN
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
const FOCAL_LENGTH: f64 = 1.0; // Orthogonal distance from the camera to the screen

// MAX COLOUR VALUE (8 bit)
const MAX_COLOUR_VALUE: i32 = 255;

// IMAGE DIRECTION VECTORS
const HORIZONTAL_DIRECTION_VECTOR: Vec3 = Vec3 { x: VIEWPORT_WIDTH, y: 0.0, z: 0.0 };
const VERTICAL_DIRECTION_VECTOR: Vec3 = Vec3 { x: 0.0, y: VIEWPORT_HEIGHT, z: 0.0 };

// IMAGE POINTS
lazy_static! {
    pub static ref IMAGE_CENTER: Point = ORIGIN - Vec3 { x: 0.0, y: 0.0, z: FOCAL_LENGTH };
    pub static ref IMAGE_LOWER_LEFT_CORNER: Point = *IMAGE_CENTER
        - HORIZONTAL_DIRECTION_VECTOR / 2.0
        - VERTICAL_DIRECTION_VECTOR / 2.0;
}

// RAY CONSTANTS
const MAX_DEPTH: i32 = 100; // Maximum number of times rays can diffuse
fn main() {

    // prints to stdout the header encoding for ppm
    encoder::ppm_headers(IMAGE_PIXEL_WIDTH, IMAGE_PIXEL_HEIGHT, MAX_COLOUR_VALUE);

    // Create spheres to populate the world with
    let sphere1_material = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let sphere1 = Sphere::new(
        Point { x: 0.0, y: 0.0, z: -1.0 },
        0.5,
        Box::new(&sphere1_material),
    );

    let sphere2_material = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let sphere2 = Sphere::new(
        Point { x: 0.0, y: -100.5, z: -1.0 },
        100.0,
        Box::new(&sphere2_material),
    );

    let sphere3_material = Metal::new(Color::new(0.8, 0.6, 0.2));
    let sphere3 = Sphere::new(
        Point { x: 1.0, y: 0.0, z: -1.0 },
        0.5,
        Box::new(&sphere3_material),
    );

    let sphere4_material = Metal::new(Color::new(0.8, 0.8, 0.8));
    let sphere4 = Sphere::new(
        Point { x: -1.0, y: 0.0, z: -1.0 },
        0.5,
        Box::new(&sphere4_material),
    );

    let world = World::new( vec![&sphere1, &sphere2, &sphere3, &sphere4] );

    // Initialize camera
    let camera = Camera::new(ASPECT_RATIO, VIEWPORT_HEIGHT, VIEWPORT_WIDTH, FOCAL_LENGTH, ORIGIN);

    // Write the pixels from top to bottom row
    for height in (0..IMAGE_PIXEL_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", height);
        // Write the pixels for each row from left to right
        for width in 0..IMAGE_PIXEL_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _samples in 0..SAMPLES_PER_PIXEL {
                // Horizontal direction vector
                let u = (width as f64 + rand::random::<f64>()) / (IMAGE_PIXEL_WIDTH - 1) as f64;

                // Vertical direction vector
                let v = (height as f64 + rand::random::<f64>()) / (IMAGE_PIXEL_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(ray, &world, MAX_DEPTH);
            }
            pixel_color.encode_as_ppm_pixel(SAMPLES_PER_PIXEL);
        }
    }
    eprintln!("\nDone.\n")
}

fn ray_color(ray: Ray, world: &World, depth: i32) -> Vec3 {
    let unit_color = Color { x: 1.0, y: 1.0, z: 1.0 };

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    // TODO Use more efficient way to find nearest point
    // TODO Refactor this to be a ray event, on hitting world object
    match world.nearest_point(ray) {
        Some ((t, normal, material)) => {
            if let Some (scattered_ray) = material.scatter(ray, t, normal) {
                return material.get_albedo() * ray_color(scattered_ray, world, depth - 1);
            }
            return Color::new(0.0, 0.0, 0.0);
        },

        None => {
            let unit_direction = ray.direction.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);
            return (1.0 - t) * unit_color
                + t * Color { x: 0.5, y: 0.7, z: 1.0 };
        }
    }
}
