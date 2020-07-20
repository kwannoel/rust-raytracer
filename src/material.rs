extern crate rand;
use rand::Rng;

use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    // Amount of light reflected
    fn get_albedo (&self) -> Color;

    // Ray absorbed -> None
    // Ray scattered -> Some (Scattered Ray)
    fn scatter (&self, ray: Ray, t: f64, normal: Vec3) -> Option<Ray>;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn get_albedo (&self) -> Color {
        self.albedo
    }

    fn scatter(&self,
               ray: Ray,
               t: f64,
               normal: Vec3)
               -> Option<Ray> {
        let scatter_direction = normal + Vec3::random_unit_vector();
        let scattered_ray = Ray::new(ray.at(t), scatter_direction);
        return Some(scattered_ray);
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let corrected_fuzz = if fuzz < 1.0 {
            fuzz
        } else {
            1.0
        };
        Self { albedo , fuzz: corrected_fuzz }
    }
}

impl Material for Metal {
    fn get_albedo (&self) -> Color {
        self.albedo
    }

    fn scatter(&self,
               ray: Ray,
               t: f64,
               normal: Vec3)
               -> Option<Ray> {
        let reflected_ray_direction = ray.direction.unit_vector().reflect(normal);
        let scattered_ray_direction = reflected_ray_direction + self.fuzz * Vec3::random_point_in_unit_sphere();
        let reflected_ray = Ray::new(ray.at(t), scattered_ray_direction);
        if reflected_ray_direction.dot(normal) > 0.0 {
            return Some(reflected_ray);
        }
        None
    }
}

#[derive(Clone)]
pub struct Dielectric {
    albedo: Color,
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { albedo: Color::new(1.0, 1.0, 1.0), refractive_index }
    }
}

impl Material for Dielectric {
    fn get_albedo (&self) -> Color {
        self.albedo
    }

    fn scatter(&self,
               ray: Ray,
               t: f64,
               normal: Vec3)
               -> Option<Ray> {
        let unit_direction = ray.direction.unit_vector();

        // normal is always outward
        // Check if the ray is inside or outside the sphere
        let is_inside = unit_direction.dot(normal) > 0.0;

        // Ensure normal used is always against the incident ray
        let opposite_normal = if is_inside { -normal } else { normal };

        // If the ray is coming from within, use the object's refractive index
        let refractive_index = if is_inside { self.refractive_index } else { 1.0 / self.refractive_index };

        let cos_theta = f64::min((-unit_direction).dot(opposite_normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if refractive_index * sin_theta > 1.0 {
            let reflected_ray_direction = unit_direction.reflect(opposite_normal);
            let scattered_ray = Ray::new(ray.at(t), reflected_ray_direction);
            return Some(scattered_ray);
        }

        // Schlick
        let reflect_probability = schlick(cos_theta, refractive_index);
        let mut rng = rand::thread_rng();

        if rng.gen_range(0.0, 1.0)< reflect_probability {
            let reflected_ray_direction = unit_direction.reflect(opposite_normal);
            let scattered_ray = Ray::new(ray.at(t), reflected_ray_direction);
            return Some(scattered_ray);
        }

        let refracted_direction_vector = unit_direction.refract(opposite_normal, refractive_index);
        let scattered_ray = Ray::new(ray.at(t), refracted_direction_vector);
        return Some(scattered_ray);
    }
}

fn schlick(cosine: f64, refractive_index: f64) -> f64 {
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
    let r0_squared = r0 * r0;
    return r0_squared + (1.0 - r0_squared) * (1.0 - cosine).powi(5);
}
