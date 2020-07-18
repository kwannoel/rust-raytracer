extern crate rand;
use rand::Rng;

use crate::world::World;
use crate::hittable::Hittable;
use crate::material::{Material, Metal, Lambertian, Dielectric};
use crate::color::Color;
use crate::sphere::Sphere;
use crate::point::Point;

pub fn random_scene<'a>() -> World<'a> {
    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    let ground_sphere = Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Box::new(&ground_material)
    );
    let mut rng = rand::thread_rng();
    let mut random_double = | | rng.gen_range(0.0, 1.0);

    let mut world_objects: Vec<&'a dyn Hittable<'a>> = vec![&ground_sphere];
    for a in (-11..11) {
        for b in (-11..11) {
            let choose_material = random_double();
            let center = Point::new(a as f64 + random_double(), 0.2, b as f64 + 0.9 * random_double());

            if ((center - Point::new(4.0, 0.2, 0.0)).length() > 0.9) {

                if (choose_material < 0.8) {
                    // diffuse material
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Lambertian::new(albedo);
                    let sphere = Sphere::<'a>::new(
                        center,
                        0.2,
                        Box::new(&sphere_material)
                    );
                    world_objects.push(&sphere);
                } else if (choose_material < 0.95) {
                    // metal
                    let albedo = Color::random() * Color::random();
                    let fuzz = rng.gen_range(0.0, 0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    let sphere = Sphere::<'a>::new(
                        center,
                        0.2,
                        Box::new(&sphere_material)
                    );
                    world_objects.push(&sphere);
                } else {
                    // glass
                    let sphere_material = Dielectric::new(1.5);
                    let sphere = Sphere::<'a>::new(
                        center,
                        0.2,
                        Box::new(&sphere_material)
                    );
                    world_objects.push(&sphere);
                }
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    let sphere1 = Sphere::<'a>::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(&material1)
    );

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    let sphere2 = Sphere::<'a>::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(&material2)
    );

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    let sphere3 = Sphere::<'a>::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(&material3)
    );

    let mut large_spheres: Vec<&'a dyn Hittable<'a>> = vec![&sphere1, &sphere2, &sphere3];
    world_objects.append(&mut large_spheres);

    World::new(world_objects)
}
