#[path = "./tests/cubesat.rs"]
mod tests;

use crate::component;
use crate::orbit;
use crate::time;
use crate::vector;
use std::fs::File;
use std::io::Write;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CubeSat {
    #[serde(default = "CubeSat::default_name")]
    pub name: Option<String>,
    #[serde(default = "CubeSat::default_active")]
    pub active: bool,
    #[serde(default = "CubeSat::default_history")]
    pub history: History,

    // Safe mode
    #[serde(default = "CubeSat::default_safe_mode")]
    pub safe_mode: bool,
    #[serde(default = "CubeSat::default_safe_limit")]
    pub safe_limit: Option<f64>,

    // Orbit
    pub orbit_type: Option<orbit::OrbitType>,
    pub orbit_parameters: Option<orbit::OrbitParameters>,
    pub time: Option<time::Time>,

    // Vectors
    #[serde(default = "CubeSat::default_vector")]
    pub pos: Option<vector::Vector3>,
    #[serde(default = "CubeSat::default_vector")]
    pub vel: Option<vector::Vector3>,
    #[serde(default = "CubeSat::default_vector")]
    pub acc: Option<vector::Vector3>,
    #[serde(default = "CubeSat::default_vector")]
    pub rot: Option<vector::Vector3>,
    #[serde(default = "CubeSat::default_vector")]
    pub rot_vel: Option<vector::Vector3>,
    #[serde(default = "CubeSat::default_vector")]
    pub rot_acc: Option<vector::Vector3>,
    #[serde(default = "CubeSat::default_sun")]
    pub sun: Option<vector::Vector3>,

    // Components
    pub solar_panels: Option<Vec<component::SolarPanel>>,
    pub eps: Option<component::Eps>,
    pub components: Option<Vec<component::Component>>,
}

impl CubeSat {
    pub fn from_toml(path: &str) -> Self {
        let file = std::fs::read_to_string(path).unwrap();
        toml::from_str(&file).unwrap()
    }

    pub fn new() -> Self {
        CubeSat {
            name: None,
            active: true,
            history: History::new(),
            safe_mode: false,
            safe_limit: None,
            orbit_type: None,
            orbit_parameters: None,
            time: None,
            pos: None,
            vel: None,
            acc: None,
            rot: None,
            rot_vel: None,
            rot_acc: None,
            sun: None,
            solar_panels: None,
            eps: None,
            components: None,
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
            "circular cosine" => self.orbit_type = Some(orbit::OrbitType::CircularCosine),
            "parametric" => self.orbit_type = Some(orbit::OrbitType::Parametric),
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
                ("altitude", r) => parameters.set_altitude(r),
                ("inclination", i) => parameters.set_inclination(i),
                ("argument of periapsis", ap) => parameters.set_argument_of_periapsis(ap),
                ("longitude of ascending node", lan) => {
                    parameters.set_longitude_of_ascending_node(lan)
                }
                ("semi-major axis", a) => parameters.set_semi_major_axis(a),
                ("eccentricity", e) => parameters.set_eccentricity(e),
                _ => {}
            }
        }
        self.orbit_parameters = Some(parameters);
        self
    }

    pub fn with_position(mut self, x: f64, y: f64, z: f64) -> Self {
        self.pos = Some(vector::Vector3::new(x, y, z));
        self
    }

    pub fn with_velocity(mut self, x: f64, y: f64, z: f64) -> Self {
        self.vel = Some(vector::Vector3::new(x, y, z));
        self
    }

    pub fn with_acceleration(mut self, x: f64, y: f64, z: f64) -> Self {
        self.acc = Some(vector::Vector3::new(x, y, z));
        self
    }

    pub fn with_rotation(mut self, x: f64, y: f64, z: f64) -> Self {
        self.rot = Some(vector::Vector3::new(x, y, z));
        self
    }

    pub fn with_rotation_velocity(mut self, x: f64, y: f64, z: f64) -> Self {
        self.rot_vel = Some(vector::Vector3::new(x, y, z));
        self
    }

    pub fn with_rotation_acceleration(mut self, x: f64, y: f64, z: f64) -> Self {
        self.rot_acc = Some(vector::Vector3::new(x, y, z));
        self
    }

    pub fn with_sun(mut self, x: f64, y: f64, z: f64) -> Self {
        self.sun = Some(vector::Vector3::new(x, y, z));
        self
    }

    pub fn with_solar_panels(
        mut self,
        orientations: Vec<(f64, f64, f64)>,
        power_generation: f64,
    ) -> Self {
        let mut solar_panels = vec![];
        for orientation in orientations {
            let panel = component::SolarPanel::new(
                power_generation,
                orientation.0,
                orientation.1,
                orientation.2,
            );
            solar_panels.push(panel);
        }
        self.solar_panels = Some(solar_panels);
        self
    }

    pub fn with_eps(mut self, power_consumption: f64, max_charge: f64) -> Self {
        self.eps = Some(component::Eps::new(power_consumption, max_charge));
        self
    }

    pub fn with_component(
        mut self,
        name: &str,
        consumption_passive: f64,
        consumption_active: Option<f64>,
        activation_interval: Option<f64>,
        activation_duration: Option<f64>,
    ) -> Self {
        match self.components {
            Some(ref mut c) => c.push(component::Component::new(
                name,
                consumption_passive,
                consumption_active,
                activation_interval,
                activation_duration,
            )),
            None => {
                self.components = Some(vec![component::Component::new(
                    name,
                    consumption_passive,
                    consumption_active,
                    activation_interval,
                    activation_duration,
                )])
            }
        }
        self
    }

    pub fn with_safety_limit(mut self, limit: f64) -> Self {
        self.safe_limit = Some(limit);
        self
    }

    pub fn update_active_components(&mut self, time: &f64, safe_mode: bool) {
        let components = self.components.as_mut().expect("No components are set!");
        for component in components {
            match (component.activation_interval, component.activation_duration) {
                // Component cannot be active
                (_, None) => continue,
                // Component can be active
                (Some(interval), Some(duration)) => {
                    // Activate component
                    if time % interval == 0.0 && !safe_mode {
                        component.active = true;
                    }
                    // Deactivate component
                    if time % interval >= duration || safe_mode {
                        component.active = false;
                    }
                }
                // Component is incorrectly set
                _ => {
                    panic!("Component {:?} is incorrectly set!", component);
                }
            }
        }
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
        let rotation = &self.rot.expect("No rotation is set!");

        // In eclipse, no power generation
        if self.in_eclipse() {
            return 0.0;
        }

        // In sun
        panels
            .iter()
            .map(|p| p.power_generation(rotation, sun))
            .sum()
    }

    pub fn get_power_consumption(&self) -> f64 {
        let mut consumption = 0.0;
        if let Some(eps) = &self.eps {
            consumption += eps.consumption;
        }
        if let Some(components) = &self.components {
            consumption += components
                .iter()
                .map(|c| match c.active {
                    true => c.consumption_active.expect("No active consumption is set!"),
                    false => c.consumption_passive,
                })
                .sum::<f64>();
        }
        consumption
    }

    pub fn battery_percentage(&self) -> f64 {
        let eps = self.eps.as_ref().expect("No EPS is set!");
        100.0 * eps.charge / eps.max_charge
    }

    pub fn update_orbit(&mut self) {
        if let Some(orbit_type) = &self.orbit_type {
            match orbit_type {
                orbit::OrbitType::CircularCosine => orbit::orbit_circular_cosine(self),
                orbit::OrbitType::Parametric => orbit::orbit_parametric(self),
            }
        } else {
            panic!("No orbit type is set!");
        }
    }

    pub fn update_rotation(&mut self) {
        // Update the rotational vectors using the Euler method

        // Guards
        let step = self.time.as_ref().expect("No time is set!").step;
        let acc = self
            .rot_acc
            .as_ref()
            .expect("No rotational acceleration is set!");
        let vel = self
            .rot_vel
            .as_mut()
            .expect("No rotational velocity is set!");
        let rot = self.rot.as_mut().expect("No rotation is set!");

        // Rotation
        rot.x += vel.x * step;
        rot.y += vel.y * step;
        rot.z += vel.z * step;

        // Velocity
        vel.x += acc.x * step;
        vel.y += acc.y * step;
        vel.z += acc.z * step;
    }

    pub fn rotate_sun(&mut self) {
        let sun = self.sun.as_mut().expect("No sun is set!");
        let step = self.time.as_ref().expect("No time is set!").step;
        let angle_per_day = 2.0 * std::f64::consts::PI / (365.25 * time::DAY) * step;

        *sun = sun.rot_z(angle_per_day);
    }

    pub fn check_safety_limit(&mut self) {
        if let Some(limit) = self.safe_limit {
            self.safe_mode = self.battery_percentage() <= limit;
        }
    }

    pub fn iterate(&mut self) {
        match self.time {
            Some(ref mut t) => {
                self.active = t.now < t.end && self.active;
                t.next();
            }
            None => panic!("No time is set!"),
        }
    }

    pub fn save_history(&mut self) {
        // Gather values
        self.history.save(
            self.time,
            self.pos,
            self.vel,
            self.acc,
            self.rot,
            self.rot_vel,
            self.rot_acc,
            self.sun,
            self.eps,
        );
    }

    pub fn simulate(&mut self) {
        // Loop until end
        while self.active {
            // Check safety limit
            self.check_safety_limit();

            // Update active components
            self.update_active_components(
                &self.time.expect("No time is set!").now,
                self.safe_mode.clone(),
            );

            // Update orbit
            self.update_orbit();

            // Update rotation
            self.update_rotation();

            // Update sun
            self.rotate_sun();

            // Save history
            self.save_history();

            // DEBUG Print
            self.print();

            // Calculate power generation
            let generation = self.get_power_generation();

            // Calculate power consumption
            let consumption = self.get_power_consumption();

            // Current net power
            let power = generation + consumption;

            // Update battery
            let step = self.time.as_ref().expect("No time is set!").step;
            let eps = self.eps.as_mut().expect("No EPS is set!");
            eps.update_capacity(power, step);

            // Next time step
            self.iterate();
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
            Some(orbit::OrbitType::CircularCosine) => println!("\t\tType: Circular cosine"),
            Some(orbit::OrbitType::Parametric) => println!("\t\tType: Parametric"),
            None => println!("\t\tNo orbit type is set!"),
        }
        match &self.orbit_parameters {
            Some(p) => {
                if let Some(r) = p.radius {
                    println!("\t\tAltitude: {} m", r);
                }
            }
            None => println!("\t\tNo orbit parameters are set!"),
        }

        // Time
        println!("\tTime:");
        match &self.time {
            Some(t) => println!(
                "\t\tNow: {} s\n\t\tStart: {} s\n\t\tEnd: {} s\n\t\tStep: {} s",
                t.now, t.start, t.end, t.step
            ),
            None => println!("\t\tNo time values have been set!"),
        }

        // Vectors
        println!("\tPosition:");
        match &self.pos {
            Some(p) => println!("\t\tx: {} m\n\t\ty: {} m\n\t\tz: {} m", p.x, p.y, p.z),
            None => println!("\t\tNo position has been set!"),
        }
        println!("\tVelocity:");
        match &self.vel {
            Some(v) => println!("\t\tx: {} m/s\n\t\ty: {} m/s\n\t\tz: {} m/s", v.x, v.y, v.z),
            None => println!("\t\tNo velocity has been set!"),
        }
        println!("\tAcceleration:");
        match &self.acc {
            Some(a) => println!(
                "\t\tx: {} m/s²\n\t\ty: {} m/s²\n\t\tz: {} m/s²",
                a.x, a.y, a.z
            ),
            None => println!("\t\tNo acceleration has been set!"),
        }
        println!("\tRotation:");
        match &self.rot {
            Some(r) => println!("\t\tx: {} deg\n\t\ty: {} deg\n\t\tz: {} deg", r.x, r.y, r.z),
            None => println!("\t\tNo rotation has been set!"),
        }
        println!("\tRotation. vel.:");
        match &self.rot_vel {
            Some(rv) => println!(
                "\t\tx: {} deg/s\n\t\ty: {} deg/s\n\t\tz: {} deg/s",
                rv.x, rv.y, rv.z
            ),
            None => println!("\t\tNo rotational velocity has been set!"),
        }
        println!("\tRotation. acc.:");
        match &self.rot_acc {
            Some(ra) => println!(
                "\t\tx: {} deg/s²\n\t\ty: {} deg/s²\n\t\tz: {} deg/s²",
                ra.x, ra.y, ra.z
            ),
            None => println!("\t\tNo rotational acceleration has been set!"),
        }
        println!("\tSun:");
        match &self.sun {
            Some(s) => println!("\t\tx: {}\n\t\ty: {}\n\t\tz: {}", s.x, s.y, s.z),
            None => println!("\t\tNo sun has been set!"),
        }

        // Components
        // Solar panels
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
        // EPS
        println!("\tEPS:");
        match &self.eps {
            Some(e) => println!(
                "\t\tConsumption: {} W\n\t\tCharge: {:.4} Wh ({:.2}%)\n\t\tMax. charge: {} Wh",
                e.consumption,
                e.charge,
                self.battery_percentage(),
                e.max_charge
            ),
            None => println!("\t\tNo EPS has been set!"),
        }
        // Generic components
        println!("\tComponents:");
        match &self.components {
            Some(components) => {
                for c in components {
                    c.print();
                }
            }
            None => println!("\t\tNo components have been set!"),
        }
    }

    // Default values for deserialization
    fn default_name() -> Option<String> {
        Some("CubeSat".to_string())
    }
    fn default_active() -> bool {
        true
    }
    fn default_history() -> History {
        History::new()
    }
    fn default_safe_mode() -> bool {
        false
    }
    fn default_vector() -> Option<vector::Vector3> {
        Some(vector::Vector3::origin())
    }
    fn default_sun() -> Option<vector::Vector3> {
        Some(vector::Vector3::new(1.0, 0.0, 0.0))
    }
    fn default_safe_limit() -> Option<f64> {
        Some(0.0)
    }
}

#[derive(Debug, PartialEq, Deserialize)]
pub struct History {
    time: Vec<f64>,
    pos: Vec<(f64, f64, f64)>,
    vel: Vec<(f64, f64, f64)>,
    acc: Vec<(f64, f64, f64)>,
    rot: Vec<(f64, f64, f64)>,
    rot_vel: Vec<(f64, f64, f64)>,
    rot_acc: Vec<(f64, f64, f64)>,
    sun: Vec<(f64, f64, f64)>,
    charge: Vec<f64>,
}

impl History {
    pub fn new() -> Self {
        History {
            time: Vec::new(),
            pos: Vec::new(),
            vel: Vec::new(),
            acc: Vec::new(),
            rot: Vec::new(),
            rot_vel: Vec::new(),
            rot_acc: Vec::new(),
            sun: Vec::new(),
            charge: Vec::new(),
        }
    }

    pub fn save(
        &mut self,
        time: Option<time::Time>,
        pos: Option<vector::Vector3>,
        vel: Option<vector::Vector3>,
        acc: Option<vector::Vector3>,
        rot: Option<vector::Vector3>,
        rot_vel: Option<vector::Vector3>,
        rot_acc: Option<vector::Vector3>,
        sun: Option<vector::Vector3>,
        eps: Option<component::Eps>,
    ) {
        // Time
        if let Some(t) = time {
            self.time.push(t.now);
        }

        // Position
        if let Some(p) = pos {
            self.pos.push((p.x, p.y, p.z));
        }

        // Velocity
        if let Some(v) = vel {
            self.vel.push((v.x, v.y, v.z));
        }

        // Acceleration
        if let Some(a) = acc {
            self.acc.push((a.x, a.y, a.z));
        }

        // Rotation
        if let Some(r) = rot {
            self.rot.push((r.x, r.y, r.z));
        }

        // Rotational velocity
        if let Some(rv) = rot_vel {
            self.rot_vel.push((rv.x, rv.y, rv.z));
        }

        // Rotational acceleration
        if let Some(ra) = rot_acc {
            self.rot_acc.push((ra.x, ra.y, ra.z));
        }

        // Sun
        if let Some(s) = sun {
            self.sun.push((s.x, s.y, s.z));
        }

        // Charge
        if let Some(e) = eps {
            self.charge.push(e.charge);
        }
    }

    pub fn write(&self, file_name: &str) {
        // Open file
        let file = File::create(file_name);
        if let Err(e) = file {
            println!("File could not be opened due to \"{}\"!", e);
            return;
        }

        // Write header
        let header = format!("time|position|velocity|acceleration|rotation|rotational velocity|rotational acceleration|sun|charge\n");
        let result = file.as_ref().unwrap().write_all(&header.into_bytes());
        if let Err(e) = result {
            println!("File could not be saved due to \"{}\"!", e);
            return;
        }

        // Write content
        for i in 0..self.time.len() {
            // Collect values
            let time = self.time[i];
            let pos = format!("{},{},{}", self.pos[i].0, self.pos[i].1, self.pos[i].2);
            let vel = format!("{},{},{}", self.vel[i].0, self.vel[i].1, self.vel[i].2);
            let acc = format!("{},{},{}", self.acc[i].0, self.acc[i].1, self.acc[i].2);
            let rot = format!("{},{},{}", self.rot[i].0, self.rot[i].1, self.rot[i].2);
            let rot_vel = format!(
                "{},{},{}",
                self.rot_vel[i].0, self.rot_vel[i].1, self.rot_vel[i].2
            );
            let rot_acc = format!(
                "{},{},{}",
                self.rot_acc[i].0, self.rot_acc[i].1, self.rot_acc[i].2
            );
            let sun = format!("{},{},{}", self.sun[i].0, self.sun[i].1, self.sun[i].2);
            let charge = self.charge[i];
            // Format line and write
            let line = format!(
                "{}|{}|{}|{}|{}|{}|{}|{}|{}\n",
                time, pos, vel, acc, rot, rot_vel, rot_acc, sun, charge
            );
            let result = file.as_ref().unwrap().write_all(&line.into_bytes());
            if let Err(e) = result {
                println!("File could not be saved due to \"{}\"!", e);
                return;
            }
        }

        println!("File '{}' was written successfully!", file_name);
    }
}
