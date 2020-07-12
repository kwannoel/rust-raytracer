extern crate rand;
use rand::Rng;

use crate::vec3::Vec3;

pub type Point = Vec3;

impl Point {
    // Generate a random point with bounds
    pub fn bound_random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        return Self::new(rng.gen_range(min, max), rng.gen_range(min, max), rng.gen_range(min, max));
    }
}
