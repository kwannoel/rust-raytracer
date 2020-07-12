use crate::vec3::Vec3;
use crate::point::Point;

// P(t) = origin + t * direction_vector
#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Point, // A
    pub direction: Vec3, // b
}

pub enum RayLocale {
    Inside, // Inside an object
    Outside, // Outside an object
}

// P(t)
impl Ray {
    pub fn at(self, t: f64) -> Point {
        self.origin + t * self.direction
    }

    // Check ray locale
    fn ray_locale(&self, outward_normal: Vec3) -> RayLocale {
        if self.direction.dot(outward_normal) > 0.0 {
            return RayLocale::Inside;
        }
        return RayLocale::Outside;
    }
}
