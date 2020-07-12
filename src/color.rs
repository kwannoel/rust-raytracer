use crate::vec3::Vec3;
use crate::utils;

pub type Color = Vec3;

impl Color {
    pub fn encode_as_ppm_pixel(&self, samples_per_pixel: i32) {
        // Divide the color total by number of samples
        let scale = 1.0 / samples_per_pixel as f64;
        let r = self.x * scale;
        let g = self.y * scale;
        let b = self.z * scale;

        // [0, 255] bit value of each color component
        let r_bits = (256.0 * utils::clamp(r, 0.0, 0.999)) as i32;
        let g_bits = (256.0 * utils::clamp(g, 0.0, 0.999)) as i32;
        let b_bits = (256.0 * utils::clamp(b, 0.0, 0.999)) as i32;

        println!("{} {} {}", r_bits, g_bits, b_bits);
    }
}
