#[derive(Debug, PartialEq, Clone, Copy)]
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

    pub fn new(vec: (f64, f64, f64)) -> Self {
        Vector3 {
            x: vec.0,
            y: vec.1,
            z: vec.2,
        }
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn abs(&self) -> f64 {
        let inner = self.x.powi(2) + self.y.powi(2) + self.z.powi(2);
        inner.sqrt()
    }

    pub fn angle_to(&self, other: &Vector3) -> f64 {
        (self.dot(other) / (other.abs() * other.abs())).acos()
    }

    pub fn rot_x(self, angle: f64) -> Self {
        Vector3 {
            x: self.x,
            y: angle.cos() * self.y - angle.sin() * self.z,
            z: angle.sin() * self.y + angle.cos() * self.z,
        }
    }

    pub fn rot_y(self, angle: f64) -> Self {
        Vector3 {
            x: angle.cos() * self.x + angle.sin() * self.z,
            y: self.y,
            z: -angle.sin() * self.x + angle.cos() * self.z,
        }
    }

    pub fn rot_z(self, angle: f64) -> Self {
        Vector3 {
            x: angle.cos() * self.x - angle.sin() * self.y,
            y: angle.sin() * self.x + angle.cos() * self.y,
            z: self.z,
        }
    }

    pub fn with_rotation(&self, rotation: &Vector3) -> Self {
        self.clone()
            .rot_x(rotation.x)
            .rot_y(rotation.y)
            .rot_z(rotation.z)
    }

    pub fn negative(&self) -> Self {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

pub fn foo() {
    println!("Hello from vector.rs");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origin() {
        let origin = Vector3::origin();
        assert_eq!(origin.x, 0.0);
        assert_eq!(origin.y, 0.0);
        assert_eq!(origin.z, 0.0);
    }

    #[test]
    fn unit_vectors() {
        let unit_x = Vector3::new((1.0, 0.0, 0.0));
        let unit_y = Vector3::new((0.0, 1.0, 0.0));
        let unit_z = Vector3::new((0.0, 0.0, 1.0));
        assert_eq!(unit_x.x, 1.0);
        assert_eq!(unit_y.y, 1.0);
        assert_eq!(unit_z.z, 1.0);
    }

    #[test]
    fn negative() {
        let origin = Vector3::origin();
        let pos_unit_x = Vector3::new((1.0, 0.0, 0.0));
        let pos_unit_y = Vector3::new((0.0, 1.0, 0.0));
        let pos_unit_z = Vector3::new((0.0, 0.0, 1.0));
        let pos_all_one = Vector3::new((1.0, 1.0, 1.0));
        let neg_unit_x = Vector3::new((-1.0, 0.0, 0.0));
        let neg_unit_y = Vector3::new((0.0, -1.0, 0.0));
        let neg_unit_z = Vector3::new((0.0, 0.0, -1.0));
        let neg_all_one = Vector3::new((-1.0, -1.0, -1.0));
        assert_eq!(origin.negative(), origin);
        assert_eq!(pos_unit_x.negative(), neg_unit_x);
        assert_eq!(pos_unit_y.negative(), neg_unit_y);
        assert_eq!(pos_unit_z.negative(), neg_unit_z);
        assert_eq!(pos_all_one.negative(), neg_all_one);
    }

    #[test]
    fn abs() {
        let origin = Vector3::origin();
        let pos_unit_x = Vector3::new((1.0, 0.0, 0.0));
        let pos_unit_y = Vector3::new((0.0, 1.0, 0.0));
        let pos_unit_z = Vector3::new((0.0, 0.0, 1.0));
        let pos_all_one = Vector3::new((1.0, 1.0, 1.0));
        let neg_unit_x = Vector3::new((-1.0, 0.0, 0.0));
        let neg_unit_y = Vector3::new((0.0, -1.0, 0.0));
        let neg_unit_z = Vector3::new((0.0, 0.0, -1.0));
        let neg_all_one = Vector3::new((-1.0, -1.0, -1.0));

        assert_eq!(origin.abs(), 0.0);
        assert_eq!(pos_unit_x.abs(), 1.0);
        assert_eq!(pos_unit_y.abs(), 1.0);
        assert_eq!(pos_unit_z.abs(), 1.0);
        assert_eq!(pos_all_one.abs(), 3.0_f64.sqrt());
        assert_eq!(neg_unit_x.abs(), 1.0);
        assert_eq!(neg_unit_y.abs(), 1.0);
        assert_eq!(neg_unit_z.abs(), 1.0);
        assert_eq!(neg_all_one.abs(), 3.0_f64.sqrt());
    }

    #[test]
    fn dot() {
        let origin = Vector3::origin();
        let pos_unit_x = Vector3::new((1.0, 0.0, 0.0));
        let pos_unit_y = Vector3::new((0.0, 1.0, 0.0));
        let pos_unit_z = Vector3::new((0.0, 0.0, 1.0));
        let pos_all_one = Vector3::new((1.0, 1.0, 1.0));
        let neg_unit_x = Vector3::new((-1.0, 0.0, 0.0));
        let neg_unit_y = Vector3::new((0.0, -1.0, 0.0));
        let neg_unit_z = Vector3::new((0.0, 0.0, -1.0));
        let neg_all_one = Vector3::new((-1.0, -1.0, -1.0));

        assert_eq!(origin.dot(&origin), 0.0);

        assert_eq!(pos_unit_x.dot(&pos_unit_x), 1.0);
        assert_eq!(pos_unit_x.dot(&pos_unit_y), 0.0);
        assert_eq!(pos_unit_x.dot(&pos_unit_z), 0.0);
        assert_eq!(pos_unit_x.dot(&pos_all_one), 1.0);
        assert_eq!(pos_unit_x.dot(&origin), 0.0);
        assert_eq!(pos_unit_y.dot(&pos_unit_x), 0.0);
        assert_eq!(pos_unit_y.dot(&pos_unit_y), 1.0);
        assert_eq!(pos_unit_y.dot(&pos_unit_z), 0.0);
        assert_eq!(pos_unit_y.dot(&pos_all_one), 1.0);
        assert_eq!(pos_unit_y.dot(&origin), 0.0);
        assert_eq!(pos_unit_z.dot(&pos_unit_x), 0.0);
        assert_eq!(pos_unit_z.dot(&pos_unit_y), 0.0);
        assert_eq!(pos_unit_z.dot(&pos_unit_z), 1.0);
        assert_eq!(pos_unit_z.dot(&pos_all_one), 1.0);
        assert_eq!(pos_unit_z.dot(&origin), 0.0);

        assert_eq!(neg_unit_x.dot(&neg_unit_x), 1.0);
        assert_eq!(neg_unit_x.dot(&neg_unit_y), 0.0);
        assert_eq!(neg_unit_x.dot(&neg_unit_z), 0.0);
        assert_eq!(neg_unit_x.dot(&neg_all_one), 1.0);
        assert_eq!(neg_unit_x.dot(&origin), 0.0);
        assert_eq!(neg_unit_y.dot(&neg_unit_x), 0.0);
        assert_eq!(neg_unit_y.dot(&neg_unit_y), 1.0);
        assert_eq!(neg_unit_y.dot(&neg_unit_z), 0.0);
        assert_eq!(neg_unit_y.dot(&neg_all_one), 1.0);
        assert_eq!(neg_unit_y.dot(&origin), 0.0);
        assert_eq!(neg_unit_z.dot(&neg_unit_x), 0.0);
        assert_eq!(neg_unit_z.dot(&neg_unit_y), 0.0);
        assert_eq!(neg_unit_z.dot(&neg_unit_z), 1.0);
        assert_eq!(neg_unit_z.dot(&neg_all_one), 1.0);
        assert_eq!(neg_unit_z.dot(&origin), 0.0);

        assert_eq!(pos_unit_x.dot(&neg_unit_x), -1.0);
        assert_eq!(pos_unit_x.dot(&neg_unit_y), 0.0);
        assert_eq!(pos_unit_x.dot(&neg_unit_z), 0.0);
        assert_eq!(pos_unit_x.dot(&neg_all_one), -1.0);
        assert_eq!(pos_unit_y.dot(&neg_unit_x), 0.0);
        assert_eq!(pos_unit_y.dot(&neg_unit_y), -1.0);
        assert_eq!(pos_unit_y.dot(&neg_unit_z), 0.0);
        assert_eq!(pos_unit_y.dot(&neg_all_one), -1.0);
        assert_eq!(pos_unit_z.dot(&neg_unit_x), 0.0);
        assert_eq!(pos_unit_z.dot(&neg_unit_y), 0.0);
        assert_eq!(pos_unit_z.dot(&neg_unit_z), -1.0);
        assert_eq!(pos_unit_z.dot(&neg_all_one), -1.0);

        assert_eq!(pos_all_one.dot(&pos_all_one), 3.0);
        assert_eq!(pos_all_one.dot(&neg_all_one), -3.0);
        assert_eq!(neg_all_one.dot(&neg_all_one), 3.0);
        assert_eq!(pos_all_one.dot(&origin), 0.0);
        assert_eq!(neg_all_one.dot(&origin), 0.0);
    }

    #[test]
    fn angle_to() {}

    #[test]
    fn rot_x() {}

    #[test]
    fn rot_y() {}

    #[test]
    fn rot_z() {}

    #[test]
    fn with_rotation() {}
}
