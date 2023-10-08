pub static RADIUS_EARTH: f64 = 6.3781e6; // [m]

#[derive(Debug, PartialEq)]
pub enum OrbitType {
    EquatorialCosine,
}

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_none() {
        let none = OrbitParameters::new();
        assert_eq!(none.radius, Option::None);
    }

    #[test]
    fn five_hundred_kilometers() {
        let mut five_hundred = OrbitParameters::new();
        five_hundred.set_radius(500_000.0);
        assert_eq!(five_hundred.radius, Option::Some(RADIUS_EARTH + 500_000.0))
    }
}
