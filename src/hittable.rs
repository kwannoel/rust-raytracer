use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Hittable {
    // When a ray is projected on the surface on the object
    // It returns the array of t-values for which the ray intersects with the surface of the object
    // and the corresponding normals
    fn hit (&self, ray: Ray) -> Vec<(f64, Vec3)>;
}