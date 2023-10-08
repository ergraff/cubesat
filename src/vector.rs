#[derive(Debug, PartialEq)]
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
}
