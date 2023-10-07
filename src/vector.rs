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
