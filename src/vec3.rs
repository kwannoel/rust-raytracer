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
        let length = self.length();
        Self {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
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