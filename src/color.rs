use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn encode_as_ppm_pixel(&self) {
        let r = (self.x * 255.999) as i32;
        let g = (self.y * 255.999) as i32;
        let b = (self.z * 255.999) as i32;
        println!("{} {} {}", r, g, b);
    }
}
