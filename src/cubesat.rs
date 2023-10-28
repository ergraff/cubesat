use crate::component;
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
    pub sun: Option<vector::Vector3>,
    pub solar_panels: Option<Vec<component::SolarPanel>>,
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
            sun: None,
            solar_panels: None,
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

    pub fn with_solar_panels(
        mut self,
        orientations: Vec<(f64, f64, f64)>,
        power_generation: f64,
    ) -> Self {
        let mut solar_panels = vec![];
        for orientation in orientations {
            let panel = component::SolarPanel::new(power_generation, orientation);
            solar_panels.push(panel);
        }
        self.solar_panels = Some(solar_panels);
        self
    }

    pub fn with_sun(mut self, sun: (f64, f64, f64)) -> Self {
        self.sun = Some(vector::Vector3::new(sun));
        self
    }

    pub fn in_eclipse(&self) -> bool {
        // Guards
        let sun = &self.sun.expect("No sun is set!");
        let pos = &self.pos.expect("No position vector is set!");

        // The conditions are:
        // 1. 0 <= |pos| * sin(acos(( dot(pos,sun)) / (|pos|*|sun|) ))  <= RADIUS_EARTH
        // 2. Angle between position and sun vectors is less than PI/2
        let pos_dot_sun = pos.dot(sun);
        let pos_times_sun = pos.abs() * sun.abs();
        let inner = pos_dot_sun / pos_times_sun;

        // Evaluate angle
        let angle = inner.acos();
        if angle.is_nan() {
            panic!("angle is NaN!");
        }

        // Evaluate result
        let result = pos.abs() * angle.sin();
        (0.0 <= result && result <= orbit::RADIUS_EARTH) && (angle < std::f64::consts::FRAC_PI_2)
    }

    pub fn get_power_generation(&self) -> f64 {
        // Guards
        let panels = self
            .solar_panels
            .as_ref()
            .expect("No solar panels are set!");
        let sun = &self.sun.expect("No sun is set!");

        // In eclipse, no power generation
        if self.in_eclipse() {
            return 0.0;
        }

        // In sun
        panels.iter().map(|p| p.power_generation(sun)).sum()
    }

    pub fn update_orbit(&mut self) {
        if let Some(orbit_type) = &self.orbit_type {
            match orbit_type {
                orbit::OrbitType::EquatorialCosine => orbit::orbit_equatorial_cosine(self),
            }
        } else {
            panic!("No orbit type is set!");
        }
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
        println!("\tSun:");
        match &self.sun {
            Some(s) => println!("\t\tx: {}\n\t\ty: {}\n\t\tz: {}", s.x, s.y, s.z),
            None => println!("No sun has been set!"),
        }

        // Components
        let number = match &self.solar_panels {
            Some(v) => v.iter().len(),
            None => 0,
        };
        println!("\tSolar panels ({}x):", number);
        if let Some(panels) = &self.solar_panels {
            for panel in panels {
                println!(
                    "\t\t({}, {}, {}), {} W",
                    panel.orientation.x,
                    panel.orientation.y,
                    panel.orientation.z,
                    panel.power_generation
                );
            }
        }
    }
}

pub fn foo() {
    println!("Hello from cubesat.rs");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let cubesat = CubeSat::new();
        assert_eq!(cubesat.name, Option::None);
        assert_eq!(cubesat.orbit_type, Option::None);
        assert_eq!(cubesat.orbit_parameters, Option::None);
        assert_eq!(cubesat.time, Option::None);
        assert_eq!(cubesat.pos, Option::None);
        assert_eq!(cubesat.vel, Option::None);
        assert_eq!(cubesat.vel, Option::None);
        assert_eq!(cubesat.rot, Option::None);
    }

    #[test]
    fn with_name() {
        let cubesat = CubeSat::new().with_name("Hello, world!");
        assert_ne!(cubesat.name, None);
        assert_eq!(cubesat.name.clone().unwrap(), "Hello, world!".to_string());
    }

    #[test]
    fn with_time() {
        let cubesat = CubeSat::new().with_time(0.0, 100.0, 1.0);
        assert_ne!(cubesat.time, None);
        assert_eq!(cubesat.time.as_ref().unwrap().now, 0.0);
        assert_eq!(cubesat.time.as_ref().unwrap().start, 0.0);
        assert_eq!(cubesat.time.as_ref().unwrap().end, 100.0);
        assert_eq!(cubesat.time.as_ref().unwrap().step, 1.0);
    }

    #[test]
    fn with_orbit_type() {
        let cubesat = CubeSat::new().with_orbit_type("equatorial cosine");
        assert_ne!(cubesat.orbit_type, None);
        assert_eq!(
            cubesat.orbit_type.unwrap(),
            orbit::OrbitType::EquatorialCosine
        );
    }

    #[test]
    fn with_orbit_parameters() {
        let cubesat = CubeSat::new().with_orbit_parameters(vec![("radius", 500_000.0)]);
        assert_ne!(cubesat.orbit_parameters, None);
        assert_eq!(
            cubesat.orbit_parameters.as_ref().unwrap().radius.unwrap(),
            orbit::RADIUS_EARTH + 500_000.0
        );
    }

    #[test]
    fn with_position() {
        let cubesat_x = CubeSat::new().with_position((1.0, 0.0, 0.0));
        let cubesat_y = CubeSat::new().with_position((0.0, 1.0, 0.0));
        let cubesat_z = CubeSat::new().with_position((0.0, 0.0, 1.0));
        assert_ne!(cubesat_x.pos, None);
        assert_ne!(cubesat_y.pos, None);
        assert_ne!(cubesat_z.pos, None);
        assert_eq!(cubesat_x.pos.as_ref().unwrap().x, 1.0);
        assert_eq!(cubesat_x.pos.as_ref().unwrap().y, 0.0);
        assert_eq!(cubesat_x.pos.as_ref().unwrap().z, 0.0);
        assert_eq!(cubesat_y.pos.as_ref().unwrap().x, 0.0);
        assert_eq!(cubesat_y.pos.as_ref().unwrap().y, 1.0);
        assert_eq!(cubesat_y.pos.as_ref().unwrap().z, 0.0);
        assert_eq!(cubesat_z.pos.as_ref().unwrap().x, 0.0);
        assert_eq!(cubesat_z.pos.as_ref().unwrap().y, 0.0);
        assert_eq!(cubesat_z.pos.as_ref().unwrap().z, 1.0);
    }

    #[test]
    fn with_velocity() {
        let cubesat_x = CubeSat::new().with_velocity((1.0, 0.0, 0.0));
        let cubesat_y = CubeSat::new().with_velocity((0.0, 1.0, 0.0));
        let cubesat_z = CubeSat::new().with_velocity((0.0, 0.0, 1.0));
        assert_ne!(cubesat_x.vel, None);
        assert_ne!(cubesat_y.vel, None);
        assert_ne!(cubesat_z.vel, None);
        assert_eq!(cubesat_x.vel.as_ref().unwrap().x, 1.0);
        assert_eq!(cubesat_x.vel.as_ref().unwrap().y, 0.0);
        assert_eq!(cubesat_x.vel.as_ref().unwrap().z, 0.0);
        assert_eq!(cubesat_y.vel.as_ref().unwrap().x, 0.0);
        assert_eq!(cubesat_y.vel.as_ref().unwrap().y, 1.0);
        assert_eq!(cubesat_y.vel.as_ref().unwrap().z, 0.0);
        assert_eq!(cubesat_z.vel.as_ref().unwrap().x, 0.0);
        assert_eq!(cubesat_z.vel.as_ref().unwrap().y, 0.0);
        assert_eq!(cubesat_z.vel.as_ref().unwrap().z, 1.0);
    }

    #[test]
    fn with_acceleration() {
        let cubesat_x = CubeSat::new().with_acceleration((1.0, 0.0, 0.0));
        let cubesat_y = CubeSat::new().with_acceleration((0.0, 1.0, 0.0));
        let cubesat_z = CubeSat::new().with_acceleration((0.0, 0.0, 1.0));
        assert_ne!(cubesat_x.acc, None);
        assert_ne!(cubesat_y.acc, None);
        assert_ne!(cubesat_z.acc, None);
        assert_eq!(cubesat_x.acc.as_ref().unwrap().x, 1.0);
        assert_eq!(cubesat_x.acc.as_ref().unwrap().y, 0.0);
        assert_eq!(cubesat_x.acc.as_ref().unwrap().z, 0.0);
        assert_eq!(cubesat_y.acc.as_ref().unwrap().x, 0.0);
        assert_eq!(cubesat_y.acc.as_ref().unwrap().y, 1.0);
        assert_eq!(cubesat_y.acc.as_ref().unwrap().z, 0.0);
        assert_eq!(cubesat_z.acc.as_ref().unwrap().x, 0.0);
        assert_eq!(cubesat_z.acc.as_ref().unwrap().y, 0.0);
        assert_eq!(cubesat_z.acc.as_ref().unwrap().z, 1.0);
    }

    #[test]
    fn with_rotation() {
        let cubesat_x = CubeSat::new().with_rotation((1.0, 0.0, 0.0));
        let cubesat_y = CubeSat::new().with_rotation((0.0, 1.0, 0.0));
        let cubesat_z = CubeSat::new().with_rotation((0.0, 0.0, 1.0));
        assert_ne!(cubesat_x.rot, None);
        assert_ne!(cubesat_y.rot, None);
        assert_ne!(cubesat_z.rot, None);
        assert_eq!(cubesat_x.rot.as_ref().unwrap().x, 1.0);
        assert_eq!(cubesat_x.rot.as_ref().unwrap().y, 0.0);
        assert_eq!(cubesat_x.rot.as_ref().unwrap().z, 0.0);
        assert_eq!(cubesat_y.rot.as_ref().unwrap().x, 0.0);
        assert_eq!(cubesat_y.rot.as_ref().unwrap().y, 1.0);
        assert_eq!(cubesat_y.rot.as_ref().unwrap().z, 0.0);
        assert_eq!(cubesat_z.rot.as_ref().unwrap().x, 0.0);
        assert_eq!(cubesat_z.rot.as_ref().unwrap().y, 0.0);
        assert_eq!(cubesat_z.rot.as_ref().unwrap().z, 1.0);
    }

    #[test]
    fn with_solar_panels() {
        let cubesat = CubeSat::new().with_solar_panels(
            vec![
                (1.0, 0.0, 0.0),
                (-1.0, 0.0, 0.0),
                (0.0, 1.0, 0.0),
                (0.0, -1.0, 0.0),
                (0.0, 0.0, 1.0),
                (0.0, 0.0, -1.0),
            ],
            1.0,
        );
        assert_ne!(cubesat.solar_panels, None);
        for i in 0..6 {
            assert_eq!(
                cubesat.solar_panels.as_ref().unwrap()[i].power_generation,
                1.0
            );
        }
        assert_eq!(cubesat.solar_panels.as_ref().unwrap()[0].orientation.x, 1.0);
        assert_eq!(cubesat.solar_panels.as_ref().unwrap()[2].orientation.y, 1.0);
        assert_eq!(cubesat.solar_panels.as_ref().unwrap()[4].orientation.z, 1.0);
        assert_eq!(
            cubesat.solar_panels.as_ref().unwrap()[1].orientation.x,
            -1.0
        );
        assert_eq!(
            cubesat.solar_panels.as_ref().unwrap()[3].orientation.y,
            -1.0
        );
        assert_eq!(
            cubesat.solar_panels.as_ref().unwrap()[5].orientation.z,
            -1.0
        );
    }

    #[test]
    fn with_sun() {
        let cubesat_x = CubeSat::new().with_sun((1.0, 0.0, 0.0));
        let cubesat_y = CubeSat::new().with_sun((0.0, 1.0, 0.0));
        let cubesat_z = CubeSat::new().with_sun((0.0, 0.0, 1.0));
        assert_eq!(cubesat_x.sun.as_ref().unwrap().x, 1.0);
        assert_eq!(cubesat_x.sun.as_ref().unwrap().y, 0.0);
        assert_eq!(cubesat_x.sun.as_ref().unwrap().z, 0.0);
        assert_eq!(cubesat_y.sun.as_ref().unwrap().x, 0.0);
        assert_eq!(cubesat_y.sun.as_ref().unwrap().y, 1.0);
        assert_eq!(cubesat_y.sun.as_ref().unwrap().z, 0.0);
        assert_eq!(cubesat_z.sun.as_ref().unwrap().x, 0.0);
        assert_eq!(cubesat_z.sun.as_ref().unwrap().y, 0.0);
        assert_eq!(cubesat_z.sun.as_ref().unwrap().z, 1.0);
    }

    #[test]
    fn in_eclipse() {
        // In the sun
        let radius = orbit::RADIUS_EARTH + 500_000.0;
        let cubesat_x = CubeSat::new()
            .with_sun((-1.0, 0.0, 0.0))
            .with_position((radius, 0.0, 0.0));
        let cubesat_y = CubeSat::new()
            .with_sun((-1.0, 0.0, 0.0))
            .with_position((0.0, radius, 0.0));
        let cubesat_my = CubeSat::new()
            .with_sun((-1.0, 0.0, 0.0))
            .with_position((0.0, -radius, 0.0));
        let cubesat_z = CubeSat::new()
            .with_sun((-1.0, 0.0, 0.0))
            .with_position((0.0, 0.0, radius));
        let cubesat_mz = CubeSat::new()
            .with_sun((-1.0, 0.0, 0.0))
            .with_position((0.0, 0.0, -radius));
        assert_eq!(cubesat_x.in_eclipse(), false);
        assert_eq!(cubesat_y.in_eclipse(), false);
        assert_eq!(cubesat_my.in_eclipse(), false);
        assert_eq!(cubesat_z.in_eclipse(), false);
        assert_eq!(cubesat_mz.in_eclipse(), false);

        // In the eclipse
        let cubesat_center = CubeSat::new()
            .with_sun((-1.0, 0.0, 0.0))
            .with_position((-radius, 0.0, 0.0));
        let cubesat_center_y = CubeSat::new().with_sun((-1.0, 0.0, 0.0)).with_position((
            -radius,
            orbit::RADIUS_EARTH,
            0.0,
        ));
        let cubesat_center_my = CubeSat::new().with_sun((-1.0, 0.0, 0.0)).with_position((
            -radius,
            -orbit::RADIUS_EARTH,
            0.0,
        ));
        let cubesat_center_z = CubeSat::new().with_sun((-1.0, 0.0, 0.0)).with_position((
            -radius,
            0.0,
            orbit::RADIUS_EARTH,
        ));
        let cubesat_center_mz = CubeSat::new().with_sun((-1.0, 0.0, 0.0)).with_position((
            -radius,
            0.0,
            -orbit::RADIUS_EARTH,
        ));
        assert_eq!(cubesat_center.in_eclipse(), true);
        assert_eq!(cubesat_center_y.in_eclipse(), true);
        assert_eq!(cubesat_center_my.in_eclipse(), true);
        assert_eq!(cubesat_center_z.in_eclipse(), true);
        assert_eq!(cubesat_center_mz.in_eclipse(), true);
    }

    #[test]
    fn get_power_generation() {
        // Sun changes position
        let cubesat = CubeSat::new()
            .with_position((orbit::RADIUS_EARTH + 500_000.0, 0.0, 0.0))
            .with_solar_panels(
                vec![
                    (1.0, 0.0, 0.0),
                    (-1.0, 0.0, 0.0),
                    (0.0, 1.0, 0.0),
                    (0.0, -1.0, 0.0),
                    (0.0, 0.0, 1.0),
                    (0.0, 0.0, -1.0),
                ],
                1.0,
            );
        let cubesat_x = cubesat.with_sun((1.0, 0.0, 0.0));
        assert_eq!(cubesat_x.get_power_generation(), 0.0);

        let cubesat_mx = cubesat_x.with_sun((-1.0, 0.0, 0.0));
        assert_eq!(cubesat_mx.get_power_generation(), 1.0);

        let cubesat_y = cubesat_mx.with_sun((0.0, 1.0, 0.0));
        assert_eq!(cubesat_y.get_power_generation(), 1.0);

        let cubesat_my = cubesat_y.with_sun((0.0, -1.0, 0.0));
        assert_eq!(cubesat_my.get_power_generation(), 1.0);

        let cubesat_z = cubesat_my.with_sun((0.0, 0.0, 1.0));
        assert_eq!(cubesat_z.get_power_generation(), 1.0);

        let cubesat_mz = cubesat_z.with_sun((0.0, 0.0, -1.0));
        assert_eq!(cubesat_mz.get_power_generation(), 1.0);
    }
}
