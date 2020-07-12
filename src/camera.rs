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
        aspect_ratio: f64,
        viewport_height: f64,
        viewport_width: f64,
        focal_length: f64,
        origin: Point
    ) -> Self {
        let focal_point = origin - Vec3::new(0.0, 0.0, focal_length);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        Self {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            focal_point,
            lower_left_corner: focal_point - horizontal/2.0 - vertical/2.0,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        return Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v* self.vertical - self.origin
        );
    }
}
