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

    pub fn with_altitude(mut self, altitude: f64) -> Self {
        self.radius = Some(RADIUS_EARTH + altitude);
        self
    }
}

pub fn foo() {
    println!("Hello from orbit.rs");
}
