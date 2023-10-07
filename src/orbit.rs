pub static RADIUS_EARTH: f64 = 6.3781e6; // [m]

pub enum OrbitType {
    EquatorialCosine,
}

pub struct OrbitParameters {
    // Equatorial and circular
    pub radius: Option<f64>, // [m]
}

impl OrbitParameters {
    pub fn new() -> Self {
        OrbitParameters { radius: None }
    }

    pub fn set_radius(&mut self, radius: f64) {
        self.radius = Some(RADIUS_EARTH + radius);
    }
}

pub fn foo() {
    println!("Hello from orbit.rs");
}
