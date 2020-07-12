use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(ray: Ray, t: f64, normal: Vec3, attenuation: Color, scattered: Ray) -> bool;
}
