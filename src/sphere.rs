use crate::point::Point;
use crate::ray::Ray;
use crate::utils;
use crate::hittable::Hittable;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Sphere { center, radius }
    }
    pub fn outward_normal(&self, ray: Ray, t: f64) -> Vec3 {
        (ray.at(t) - self.center).unit_vector()
    }
}

impl Hittable for Sphere
{
    fn hit(&self, ray: Ray) -> Vec<(f64, Vec3)> {
        match hit_sphere(self.center, self.radius, ray) {
            None => vec![],
            Some ((root1, root2)) => vec![root1, root2].iter().map(
                |root| (*root, *&self.outward_normal(ray, *root))
            ).collect(),
        }
    }
}

// Suppose there exists a sphere past the screen
// If a point lies the surface of the sphere,
// The distance from the point to the center of the sphere is equivalent to the sphere's radius.
//
// We can express it like so:
// Let P be the coordinate of point
// Let C be the coordinate of origin
// Then, CP = P - C (1)
// |CP| = r
// Substituting (1)
// |P - C| = r
// (P - C) · (P - C) = r^2 (2)
//
// To see if a Ray hits the surface of a sphere,
// Check if range of points for the ray satisfy (2) above
// Let p be variable vector for points
// Let O be ray origin
// Let d be direction of ray
// Then p = O + t * d (3)
//
// Substituting (3) into (2)
// (O + t * d - C) · (O + t * d - C) = r ^ 2
// (t * d + CO) · (t * d + CO) = r ^ 2
//
// We want to see if we can find a t value
// (d ^ 2 * t ^ 2)  + (2 * (CO · d) * t) + ((CO · CO) -  r ^ 2) = 0
// We recognize this is a quadratic equation with t as the subject.
// Let a = d ^ 2
// Let b = 2 * (CO · d)
// Let c = ((CO · CO) -  r ^ 2)
// Solve: a * t ^ 2 + b * t + c = 0
//
// Return t values if any
pub fn hit_sphere(center: Point, radius: f64, ray: Ray) -> Option<(f64, f64)> {
    let Ray { origin, direction } = ray;
    let co = origin - center;
    let a = direction.dot(direction);
    let b = 2.0 * (co.dot(direction));
    let c = co.dot(co) - radius * radius;
    let (root1, root2) = utils::quadratic_solver(a, b, c)?;
    if (root1 < 0.0) {
        return None;
    }
    Some((root1, root2))
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn ray_should_hit_sphere() {
        let center = Point { x: 0.0, y: -100.5, z: -1.0 };
        let radius = 100.0;
        let ray_origin = Point { x: 0.0, y: 0.0, z: 0.0};
        let ray_direction = center - ray_origin;
        let ray = Ray::new(ray_origin, ray_direction);
        let hits = hit_sphere(center, radius, ray);
        assert!(hits.is_some());
    }

    #[test]
    fn ray_reflection_should_not_hit_sphere() {
        let center = Point { x: 0.0, y: -100.5, z: -1.0 };
        let radius = 100.0;
        let ray_origin = Point { x: 0.0, y: 0.0, z: 0.0};
        let reflected_center = Point { x: 0.0, y: 100.5, z: -1.0 };
        let ray_direction = reflected_center - ray_origin;
        let ray = Ray::new(ray_origin, ray_direction);
        let hits = hit_sphere(center, radius, ray);
        assert!(hits.is_none());
    }
}
