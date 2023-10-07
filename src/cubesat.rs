use crate::orbit;
use crate::time;
use crate::vector;

pub struct CubeSat {
    pub name: Option<String>,
    pub orbit_type: Option<orbit::OrbitType>,
    pub orbit_parameters: Option<orbit::OrbitParameters>,
    pub time: Option<time::Time>,
    pub pos: Option<vector::Vector3>,
    pub vel: Option<vector::Vector3>,
    pub acc: Option<vector::Vector3>,
    pub rot: Option<vector::Vector3>,
}

impl CubeSat {
    pub fn new() -> Self {
        CubeSat {
            name: None,
            orbit_type: None,
            orbit_parameters: None,
            time: None,
            pos: None,
            vel: None,
            acc: None,
            rot: None,
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn with_orbit_type(mut self, orbit_type: &str) -> Self {
        match orbit_type {
            "equatorial cosine" => self.orbit_type = Some(orbit::OrbitType::EquatorialCosine),
            t => {
                self.orbit_type = None;
                println!("{} is not a valid orbit type!", t);
            }
        }
        self
    }

    pub fn with_orbit_parameters(mut self, orbit_parameters: Vec<(&str, f64)>) -> Self {
        let mut parameters = orbit::OrbitParameters::new();
        for p in orbit_parameters {
            match p {
                ("radius", r) => parameters.set_radius(r),
                _ => {}
            }
        }
        self.orbit_parameters = Some(parameters);
        self
    }
}

pub fn foo() {
    println!("Hello from cubesat.rs");
}
