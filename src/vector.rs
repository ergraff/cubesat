#[path = "./tests/vector.rs"]
mod tests;

use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Copy, Deserialize)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn origin() -> Self {
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn abs(&self) -> f64 {
        let inner = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        inner.sqrt()
    }

    pub fn angle_to(&self, other: &Vector3) -> f64 {
        let term1 = self.dot(other);
        let term2 = self.abs() * other.abs();
        let mut frac = term1 / term2;
        if frac < -1.0 {
            frac = -1.0;
        }
        if frac > 1.0 {
            frac = 1.0;
        }
        frac.acos() // Angle
    }

    pub fn rot_x(self, radians: f64) -> Self {
        let mut x = self.x;
        let mut y = radians.cos() * self.y - radians.sin() * self.z;
        let mut z = radians.sin() * self.y + radians.cos() * self.z;
        // Round very-near-zero to zero
        let epsilon = std::f64::EPSILON;
        if x.abs() < epsilon {
            x = 0.0;
        }
        if y.abs() < epsilon {
            y = 0.0;
        }
        if z.abs() < epsilon {
            z = 0.0;
        }
        Vector3 { x, y, z }
    }

    pub fn rot_y(self, radians: f64) -> Self {
        let mut x = radians.cos() * self.x + radians.sin() * self.z;
        let mut y = self.y;
        let mut z = -radians.sin() * self.x + radians.cos() * self.z;
        // Round very-near-zero to zero
        let epsilon = std::f64::EPSILON;
        if x.abs() < epsilon {
            x = 0.0;
        }
        if y.abs() < epsilon {
            y = 0.0;
        }
        if z.abs() < epsilon {
            z = 0.0;
        }
        Vector3 { x, y, z }
    }

    pub fn rot_z(self, radians: f64) -> Self {
        let mut x = radians.cos() * self.x - radians.sin() * self.y;
        let mut y = radians.sin() * self.x + radians.cos() * self.y;
        let mut z = self.z;
        // Round very-near-zero to zero
        let epsilon = std::f64::EPSILON;
        if x.abs() < epsilon {
            x = 0.0;
        }
        if y.abs() < epsilon {
            y = 0.0;
        }
        if z.abs() < epsilon {
            z = 0.0;
        }
        Vector3 { x, y, z }
    }

    pub fn negative(&self) -> Self {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
