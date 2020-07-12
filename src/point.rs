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

    pub fn random_point_in_unit_sphere() -> Self {
        let mut point = Point::new(0.0, 0.0, 0.0);
        while (true) {
            point = Self::bound_random(-1.0, 1.0);

            // If point is at the edges / outside the unit sphere continue to find another point
            if (point.length_squared() >= 1.0) { continue; }
            break;
        }
        // return the point
        return point;
    }
}
