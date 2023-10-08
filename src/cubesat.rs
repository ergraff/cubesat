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

    pub fn with_time(mut self, start: f64, end: f64, step: f64) -> Self {
        self.time = Some(time::Time::new(start, end, step));
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

    pub fn with_position(mut self, position: (f64, f64, f64)) -> Self {
        self.pos = Some(vector::Vector3::new(position));
        self
    }

    pub fn with_velocity(mut self, velocity: (f64, f64, f64)) -> Self {
        self.vel = Some(vector::Vector3::new(velocity));
        self
    }

    pub fn with_acceleration(mut self, acceleration: (f64, f64, f64)) -> Self {
        self.acc = Some(vector::Vector3::new(acceleration));
        self
    }

    pub fn with_rotation(mut self, rotation: (f64, f64, f64)) -> Self {
        self.rot = Some(vector::Vector3::new(rotation));
        self
    }

    pub fn print(&self) {
        // Name
        match &self.name {
            Some(n) => println!("Name: {}", n),
            None => println!("No name is set!"),
        }

        // Orbit
        println!("\tOrbit:");
        match &self.orbit_type {
            Some(orbit::OrbitType::EquatorialCosine) => println!("\t\tType: Equatorial cosine"),
            None => println!("No orbit type is set!"),
        }
        match &self.orbit_parameters {
            Some(p) => {
                if let Some(r) = p.radius {
                    println!("\t\tRadius: {} m", r);
                }
            }
            None => println!("No orbit parameters are set!"),
        }

        // Time
        println!("\tTime:");
        match &self.time {
            Some(t) => println!(
                "\t\tNow: {} s\n\t\tStart: {} s\n\t\tEnd: {} s\n\t\tStep: {} s",
                t.now, t.start, t.end, t.step
            ),
            None => println!("No time values have been set!"),
        }

        // Vectors
        println!("\tPosition:");
        match &self.pos {
            Some(p) => println!("\t\tx: {}\n\t\ty: {}\n\t\tz: {}", p.x, p.y, p.z),
            None => println!("No position has been set!"),
        }
        println!("\tVelocity:");
        match &self.vel {
            Some(v) => println!("\t\tx: {}\n\t\ty: {}\n\t\tz: {}", v.x, v.y, v.z),
            None => println!("No velocity has been set!"),
        }
        println!("\tAcceleration:");
        match &self.acc {
            Some(a) => println!("\t\tx: {}\n\t\ty: {}\n\t\tz: {}", a.x, a.y, a.z),
            None => println!("No acceleration has been set!"),
        }
        println!("\tRotation:");
        match &self.rot {
            Some(r) => println!("\t\tx: {}\n\t\ty: {}\n\t\tz: {}", r.x, r.y, r.z),
            None => println!("No rotation has been set!"),
        }
    }
}

pub fn foo() {
    println!("Hello from cubesat.rs");
}
