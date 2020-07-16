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
    albedo: Color
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
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
       let reflected_ray = Ray::new(ray.at(t), reflected_ray_direction);
       if reflected_ray_direction.dot(normal) > 0.0 {
           return Some(reflected_ray);
       }
       None
    }
}
