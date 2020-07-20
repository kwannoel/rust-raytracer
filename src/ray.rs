use crate::vec3::Vec3;
use crate::point::Point;

// P(t) = origin + t * direction_vector
#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point, // A
    pub direction: Vec3, // b
}

// P(t)
impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(self, t: f64) -> Point {
        self.origin + t * self.direction
    }
}
