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
        let refracted_direction_vector = unit_direction.refract(normal, self.refractive_index);
        let scattered_ray = Ray::new(ray.origin, refracted_direction_vector);
        return Some(scattered_ray);
    }
}
