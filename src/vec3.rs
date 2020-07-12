extern crate rand;
use rand::Rng;

use std::ops::Neg;
use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;

// Common interface for Vec3
#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/* ============
 * = UTILTIES =
 * ============
 */

impl Vec3 {
    // Constructor
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    // Dot product
    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // Cross product
    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    // Vector length squared
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    // Vector length
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    // Unit vector
    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    // Generate a random direction vector
    pub fn random() -> Self {
        return Self::new(rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>());
    }

    // Generate a random unit vector, taking the object as a lambertian surface
    pub fn random_unit_vector() -> Self {
        let mut rng = rand::thread_rng();
        // z-axis component of the incident light ray
        let z: f64 = rng.gen_range(-1.0, 1.0);

        // Using pythagoras theorem, find the length of the x + y component
        let xy = (1.0 - z * z).sqrt();

        // Generate a random angle between x and y components
        let angle = rng.gen_range(0.0, 2.0 * std::f64::consts::PI);
        let x = xy * angle.cos();
        let y = xy * angle.sin();
        return Self::new(x, y, z);
    }

    pub fn random_point_in_unit_sphere() -> Self {
        let mut point = Self::new(0.0, 0.0, 0.0);
        while (true) {
            point = Self::bound_random(-1.0, 1.0);

            // If point is at the edges / outside the unit sphere continue to find another point
            if (point.length_squared() >= 1.0) { continue; }
            break;
        }
        // return the point
        return point;
    }

    // Generate diffuse rays with no dependence on the normal
    pub fn random_in_hemisphere(normal: Vec3) -> Self {
        let in_unit_sphere = Self::random_point_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            return in_unit_sphere;
        }
        return -in_unit_sphere;
    }
}


// negate vector fields
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: - self.x,
            y: - self.y,
            z: - self.z,
        }
    }
}

// vector1 + vector2
impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Vec3) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

// vector1 - vector2
impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Vec3) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// vector1 * scalar
impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, term: f64) -> Self::Output {
        Self {
            x: self.x * term,
            y: self.y * term,
            z: self.z * term,
        }
    }
}

// scalar * vector1
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Self::Output {
        Vec3 {
            x: vec.x * self,
            y: vec.y * self,
            z: vec.z * self,
        }
    }
}

// vector1 / scalar
impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, term: f64) -> Self::Output {
        Self {
            x: self.x / term,
            y: self.y / term,
            z: self.z / term,
        }
    }
}
