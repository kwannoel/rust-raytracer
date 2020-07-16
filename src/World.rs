use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::Vec3;
use crate::material::Material;

// Objects within the world struct should have the same lifetime as the world
pub struct World <'a> {
    objects: Vec<&'a dyn Hittable<'a>>,
}

// World is hittable
impl <'a> Hittable<'a> for World<'a> {

    // Compose all hittable objects
    fn hit(&self, ray: Ray) -> Vec<(f64, Vec3, Box<&'a dyn Material>)> {
        let mut res = vec![];
        for object in self.objects.iter() {
            res.append(&mut object.hit(ray));
        }
        return res;
    }
}

impl <'a> World<'a> {
    pub fn new(objects: Vec<&'a dyn Hittable<'a>>) -> Self {
        World { objects }
    }
    // Nearest point from origin to Ray incidence will be smallest t_value
    // If no such point exists, return None
    pub fn nearest_point(&self, ray: Ray) -> Option<(f64, Vec3, Box<&'a dyn Material>)> {
        let t_values = self.hit(ray);
        let mut min = f64::MAX;
        let mut min_normal = None;
        let mut min_material = None;
        if t_values.len() > 0 {
            for (t, normal, material) in t_values.iter() {
                if *t < min {
                    min = *t;
                    min_normal = Some(*normal);
                    min_material = Some(material.clone());
                }
            }
            return Some((min, min_normal?, min_material?));
        }

        None
    }
}
