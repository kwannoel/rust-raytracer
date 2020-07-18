use crate::point::Point;
use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    // Camera image dimensions
    aspect_ratio: f64,
    viewport_height: f64,
    viewport_width: f64,

    // Camera location
    focal_length: f64,
    origin: Point,

    // Camera image vectors
    horizontal: Vec3,
    vertical: Vec3,

    // Camera image location
    focal_point: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Point, // Camera location
        look_at: Point, // Point we focus on
        vup: Vec3, // Use to determine camera tilt
        vertical_field_of_view: f64, // Vertical field of view in degrees
        aspect_ratio: f64,
        focal_length: f64,
    ) -> Self {
        let vfov_radians = vertical_field_of_view.to_radians();
        let camera_height = (vfov_radians/2.0).tan(); // Relative to y-axis = 0
        let viewport_height = 2.0 * camera_height;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = (vup.cross(w)).unit_vector();
        let v = w.cross(u);

        let origin = look_from;

        let focal_point = origin - Vec3::new(0.0, 0.0, focal_length);
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        Self {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            focal_point,
            lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - w,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v* self.vertical - self.origin
        );
    }
}
