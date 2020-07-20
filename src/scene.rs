extern crate rand;
use rand::Rng;
use std::rc::Rc;

use crate::world::World;
use crate::hittable::Hittable;
use crate::material::{Metal, Lambertian, Dielectric};
use crate::color::Color;
use crate::sphere::Sphere;
use crate::point::Point;
use crate::utils::random_probability;

pub fn random_scene() -> World {
    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let ground_sphere = Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(ground_material)
    );

    let mut rng = rand::thread_rng();

    let material1 = Dielectric::new(1.5);
    let sphere1 = Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
       Rc::new(material1)
    );

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let sphere2 = Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        Rc::new(material2)
    );

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    let sphere3 = Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        Rc::new(material3)
    );

    let mut world_objects: Vec<Box<dyn Hittable>> = vec![
        Box::new(ground_sphere),
        Box::new(sphere1),
        Box::new(sphere2),
        Box::new(sphere3)
    ];

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random_probability();
            let center = Point::new(a as f64 + random_probability(), 0.2, b as f64 + 0.9 * random_probability());

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {

                if choose_material < 0.8 {
                    // diffuse material
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);
                    let sphere = Sphere::new(
                        center,
                        0.2,
                        Rc::new(sphere_material)
                    );
                    world_objects.push(Box::new(sphere));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::random() * Color::random();
                    let fuzz = rng.gen_range(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    let sphere = Sphere::new(
                        center,
                        0.2,
                        Rc::new(sphere_material)
                    );
                    world_objects.push(Box::new(sphere));
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    let sphere = Sphere::new(
                        center,
                        0.2,
                        Rc::new(sphere_material)
                    );
                    world_objects.push(Box::new(sphere));
                }
            }
        }
    }
    World::new(world_objects)
}
