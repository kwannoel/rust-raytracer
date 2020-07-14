use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Copy, Clone)]
pub struct Material {
    pub albedo: Color,
    // Ray absorbed -> None
    // Ray scattered -> Some (Scattered Ray)
    pub scatter: fn (ray: Ray, t: f64, normal: Vec3) -> Option<Ray>,
}

impl Material {
    pub fn new(albedo: Color, scatter: fn (ray: Ray, t: f64, normal: Vec3) -> Option<Ray>) -> Self {
        Self { albedo,  scatter }
    }
    pub fn new_lambertian(albedo: Color) -> Self {
         fn scatter(ray: Ray,
                   t: f64,
                   normal: Vec3)
                   -> Option<Ray> {
            let scatter_direction = normal + Vec3::random_unit_vector();
            let scattered_ray = Ray::new(ray.at(t), scatter_direction);
            return Some(scattered_ray);
        }
        Self { albedo, scatter }
    }

    pub fn new_metal(albedo: Color) -> Self {
        fn scatter(ray: Ray,
                   t: f64,
                   normal: Vec3)
                   -> Option<Ray> {
            let reflected_ray_direction = ray.direction.unit_vector().reflect(normal);
            let reflected_ray = Ray::new(ray.at(t), reflected_ray_direction);
            if reflected_ray_direction.dot(normal) > 0.0 {
                return Some(reflected_ray);
            }
            None
        }
        Self { albedo, scatter }
    }
}
