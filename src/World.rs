use crate::hittable::Hittable;
use crate::ray::Ray;

// Objects within the world struct should have the same lifetime as the world
pub struct World <'a> {
    objects: Vec<&'a dyn Hittable>,
}

// World is hittable
impl <'a> Hittable for World<'a> {

    // Compose all hittable objects
    fn hit(&self, ray: Ray) -> Vec<f64> {
        let mut res = vec![];
        for object in self.objects.iter() {
            res.append(&mut object.hit(ray));
        }
        return res;
    }


}

impl <'a> World<'a> {
    pub fn new(objects: Vec<&'a dyn Hittable>) -> Self {
        World { objects }
    }
    // Nearest point from origin to Ray incidence will be smallest t_value
    pub fn nearest_point(&self, ray: Ray) -> Option<f64> {
        let t_values = self.hit(ray);
        let mut min = f64::MAX;
        if t_values.len() > 0 {
            for t in t_values.iter() {
                if *t < min {
                    min = *t;
                }
            }
            return Some(min);
        }

        None
    }
}
