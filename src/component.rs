#[path = "./tests/component.rs"]
mod tests;

use crate::time;
use crate::vector;

#[derive(Debug, PartialEq)]
pub struct SolarPanel {
    pub orientation: vector::Vector3,
    pub power_generation: f64, // [W]
}

impl SolarPanel {
    pub fn new(power_generation: f64, x: f64, y: f64, z: f64) -> Self {
        SolarPanel {
            orientation: vector::Vector3::new(x, y, z),
            power_generation,
        }
    }

    pub fn power_generation(&self, rotation: &vector::Vector3, sun: &vector::Vector3) -> f64 {
        // Angle
        let ang_to_rad = std::f64::consts::PI / 180.0;
        let angle = self
            .orientation
            .rot_x(rotation.x * ang_to_rad)
            .rot_y(rotation.y * ang_to_rad)
            .rot_z(rotation.z * ang_to_rad)
            .negative()
            .angle_to(sun);

        // match angle, with cosine
        match angle >= std::f64::consts::FRAC_PI_2 {
            true => 0.0,
            false => self.power_generation * angle.cos(),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Eps {
    pub consumption: f64, // [W]
    pub charge: f64,      // [Wh]
    pub max_charge: f64,  // [Wh]
}

impl Eps {
    pub fn new(power_consumption: f64, max_charge: f64) -> Self {
        Eps {
            consumption: power_consumption,
            charge: max_charge,
            max_charge,
        }
    }

    pub fn update_capacity(&mut self, power: f64, timestep: f64) {
        // W -> Joule: W * time
        let joule = power * timestep;

        // Wh -> Joule: Wh * 1 hour
        let charge_joule = self.charge * time::HOUR;

        // New value
        let new_charge_joule = charge_joule + joule;

        // Joule -> Wh: Joule / 1 hour
        let new_charge = new_charge_joule / time::HOUR;

        // Update and respect limits
        match (new_charge < 0.0, self.max_charge < new_charge) {
            // New value is negative
            (true, _) => self.charge = 0.0,
            // New value is above max limit
            (_, true) => self.charge = self.max_charge,
            // New value is in limits
            _ => self.charge = new_charge,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Component {
    pub name: String,
    pub active: bool,
    pub consumption_passive: f64,
    pub consumption_active: Option<f64>,
    pub activation_interval: Option<f64>,
    pub activation_duration: Option<f64>,
}

impl Component {
    pub fn new(
        name: &str,
        consumption_passive: f64,
        consumption_active: Option<f64>,
        activation_interval: Option<f64>,
        activation_duration: Option<f64>,
    ) -> Self {
        Component {
            name: name.to_string(),
            active: false,
            consumption_passive,
            consumption_active,
            activation_interval,
            activation_duration,
        }
    }

    pub fn print(&self) {
        let name = &self.name;
        let active = self.active;
        let consumption = match self.active {
            true => self
                .consumption_active
                .expect("No active consumption is set!"),
            false => self.consumption_passive,
        };
        println!(
            "\t\tName: {}, active: {}, consumption: {} W",
            name, active, consumption
        );
    }
}
