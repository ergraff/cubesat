use crate::orbit;
use crate::time;
use crate::vector;

pub struct CubeSat {
    pub name: String,
    pub orbit_type: orbit::OrbitType,
    pub orbit_parameters: orbit::OrbitParameters,
    pub time: time::Time,
    pub pos: vector::Vector3,
    pub vel: vector::Vector3,
    pub acc: vector::Vector3,
    pub rot: vector::Vector3,
}

impl CubeSat {
    pub fn new(name: &str, orbit_altitude: f64, time: (f64, f64, f64)) -> Self {
        // Name
        let name = name.to_string();
        // Orbit
        let orbit_type = orbit::OrbitType::EquatorialCosine;
        let orbit_parameters = orbit::OrbitParameters::new().with_altitude(orbit_altitude);
        // Time
        let time = time::Time::new(time.0, time.1, time.2);
        // Vectors

        CubeSat {
            name,
            orbit_type,
            orbit_parameters,
            time,
            pos: vector::Vector3::origin(),
            vel: vector::Vector3::origin(),
            acc: vector::Vector3::origin(),
            rot: vector::Vector3::origin(),
        }
    }
}

pub fn foo() {
    println!("Hello from cubesat.rs");
}
