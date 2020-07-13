use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    // Ray absorbed -> None
    // Ray scattered -> Some (Scattered Ray, Attenuation color)
    fn scatter(&self, ray: Ray, t: f64, normal: Vec3) -> Option<(Ray, Color)>;
}

/* =======================
 * = LAMBERTIAN MATERIAL =
 * =======================
 */

pub struct Lambertian {
    albedo: Color // Measure of how much light that hits a surface is reflected w/o being absorbed
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

}

impl Material for Lambertian {
    fn scatter(&self,
               ray: Ray,
               t: f64,
               normal: Vec3)
               -> Option<(Ray, Color)> {
        let scatter_direction = normal + Vec3::random_unit_vector();
        let scattered_ray = Ray::new(ray.at(t), scatter_direction);
        let attenuation = self.albedo;
        return Some((scattered_ray, attenuation));
    }
}

/* ==================
 * = METAL MATERIAL =
 * ==================
 */

pub struct Metal {
    albedo: Color // Measure of how much light that hits a surface is reflected w/o being absorbed
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

}

impl Material for Metal {
    fn scatter(&self,
               ray: Ray,
               t: f64,
               normal: Vec3)
               -> Option<(Ray, Color)> {
        let reflected_ray_direction = ray.direction.unit_vector().reflect(normal);
        let reflected_ray = Ray::new(ray.at(t), reflected_ray_direction);
        let attenuation = self.albedo;
        if reflected_ray_direction.dot(normal) > 0.0 {
            return Some((reflected_ray, attenuation));
        }
        None
    }
}
