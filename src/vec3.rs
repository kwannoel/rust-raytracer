extern crate rand;
use rand::Rng;

use std::ops::Neg;
use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;

// Common interface for Vec3
#[derive(Copy, Clone, Debug, PartialEq)]
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
        let mut point;
        loop {
            point = Self::bound_random(-1.0, 1.0);

            // If point is at the edges / outside the unit sphere continue to find another point
            if point.length_squared() >= 1.0 { continue; }
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

    pub fn reflect(self, normal: Vec3) -> Self {
        return self - 2.0 * self.dot(normal) * normal;
    }

    // let first_medium_indice = incident ray region's refractive indice
    // let second_medium_indice = Medium refractive indice
    //
    pub fn refract(self, normal: Vec3, refractive_index: f64) -> Vec3 {
        let normalized_self = self.unit_vector();
        let cos_theta = -normalized_self.dot(normal);
        let r_out_parallel = refractive_index * (normalized_self + cos_theta * normal);
        let r_out_perpendicular = -((1.0 - r_out_parallel.length_squared()).sqrt()) * normal;
        let r_out = r_out_parallel + r_out_perpendicular;
        r_out
    }

    pub fn random_in_unit_disk() -> Vec3 {
        let mut p: Vec3;
        let mut rng = rand::thread_rng();
        loop {
            p = Vec3::new(rng.gen_range(-1.0, 1.0), rng.gen_range(-1.0, 1.0), 0.0);
            if p.length_squared() >= 1.0 { continue; }
            break;
        }
        return p;
    }

    // Generate a random vector within specified bounds
    pub fn bound_random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        return Self::new(rng.gen_range(min, max), rng.gen_range(min, max), rng.gen_range(min, max));
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

// vector1 * vector2 (row-wise)
impl Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, other: Vec3) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_should_reflect() {
        let direction_vector = Vec3::new(1.0, 1.0, 0.0);
        let normal = Vec3::new(1.0, 0.0, 0.0);
        let reflected_vector = direction_vector.reflect(normal);
        assert_eq!(direction_vector.reflect(normal), reflected_vector);
    }

    #[test]
    fn direction_should_refract() {
        let direction_vector = Vec3::new(1.0, 1.0, 0.0);
        let normal = Vec3::new(-1.0, 0.0, 0.0);
        let refractive_index = 1.0 / 1.5; // Air -> Glass
        let refracted_vector = Vec3::new(0.881917103688197, 0.4714045207910316, 0.0);
        assert_eq!(direction_vector.refract(normal, refractive_index), refracted_vector);
    }
}
