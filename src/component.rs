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
    pub consumption: f64,
}

impl Component {
    pub fn new(name: &str, consumption: f64) -> Self {
        Component {
            name: name.to_string(),
            consumption,
        }
    }

    pub fn print(&self) {
        println!(
            "\t\tName: {}, consumption: {} W",
            self.name, self.consumption
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_new() {
        let panel_zero = SolarPanel::new(0.0, 0.0, 0.0, 0.0);
        let panel_x = SolarPanel::new(0.0, 1.0, 0.0, 0.0);
        let panel_y = SolarPanel::new(0.0, 0.0, 1.0, 0.0);
        let panel_z = SolarPanel::new(0.0, 0.0, 0.0, 1.0);
        assert_eq!(panel_zero.orientation, vector::Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(panel_x.orientation, vector::Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(panel_y.orientation, vector::Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(panel_z.orientation, vector::Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn power_generation() {
        let panel_pos_x = SolarPanel::new(1.0, 1.0, 0.0, 0.0);
        let panel_neg_x = SolarPanel::new(1.0, -1.0, 0.0, 0.0);
        let panel_pos_y = SolarPanel::new(1.0, 0.0, 1.0, 0.0);
        let panel_neg_y = SolarPanel::new(1.0, 0.0, -1.0, 0.0);
        let panel_pos_z = SolarPanel::new(1.0, 0.0, 0.0, 1.0);
        let panel_neg_z = SolarPanel::new(1.0, 0.0, 0.0, -1.0);
        let sun_pos_x = vector::Vector3::new(1.0, 0.0, 0.0);
        let sun_neg_x = vector::Vector3::new(-1.0, 0.0, 0.0);
        let sun_pos_y = vector::Vector3::new(0.0, 1.0, 0.0);
        let sun_neg_y = vector::Vector3::new(0.0, -1.0, 0.0);
        let sun_pos_z = vector::Vector3::new(0.0, 0.0, 1.0);
        let sun_neg_z = vector::Vector3::new(0.0, 0.0, -1.0);
        let rotation = vector::Vector3::new(0.0, 0.0, 0.0);

        assert_eq!(panel_pos_x.power_generation(&rotation, &sun_pos_x), 0.0);
        assert_eq!(panel_pos_x.power_generation(&rotation, &sun_neg_x), 1.0);
        assert_eq!(panel_pos_x.power_generation(&rotation, &sun_pos_y), 0.0);
        assert_eq!(panel_pos_x.power_generation(&rotation, &sun_neg_y), 0.0);
        assert_eq!(panel_pos_x.power_generation(&rotation, &sun_pos_z), 0.0);
        assert_eq!(panel_pos_x.power_generation(&rotation, &sun_neg_z), 0.0);

        assert_eq!(panel_neg_x.power_generation(&rotation, &sun_pos_x), 1.0);
        assert_eq!(panel_neg_x.power_generation(&rotation, &sun_neg_x), 0.0);
        assert_eq!(panel_neg_x.power_generation(&rotation, &sun_pos_y), 0.0);
        assert_eq!(panel_neg_x.power_generation(&rotation, &sun_neg_y), 0.0);
        assert_eq!(panel_neg_x.power_generation(&rotation, &sun_pos_z), 0.0);
        assert_eq!(panel_neg_x.power_generation(&rotation, &sun_neg_z), 0.0);

        assert_eq!(panel_pos_y.power_generation(&rotation, &sun_pos_x), 0.0);
        assert_eq!(panel_pos_y.power_generation(&rotation, &sun_neg_x), 0.0);
        assert_eq!(panel_pos_y.power_generation(&rotation, &sun_pos_y), 0.0);
        assert_eq!(panel_pos_y.power_generation(&rotation, &sun_neg_y), 1.0);
        assert_eq!(panel_pos_y.power_generation(&rotation, &sun_pos_z), 0.0);
        assert_eq!(panel_pos_y.power_generation(&rotation, &sun_neg_z), 0.0);

        assert_eq!(panel_neg_y.power_generation(&rotation, &sun_pos_x), 0.0);
        assert_eq!(panel_neg_y.power_generation(&rotation, &sun_neg_x), 0.0);
        assert_eq!(panel_neg_y.power_generation(&rotation, &sun_pos_y), 1.0);
        assert_eq!(panel_neg_y.power_generation(&rotation, &sun_neg_y), 0.0);
        assert_eq!(panel_neg_y.power_generation(&rotation, &sun_pos_z), 0.0);
        assert_eq!(panel_neg_y.power_generation(&rotation, &sun_neg_z), 0.0);

        assert_eq!(panel_pos_z.power_generation(&rotation, &sun_pos_x), 0.0);
        assert_eq!(panel_pos_z.power_generation(&rotation, &sun_neg_x), 0.0);
        assert_eq!(panel_pos_z.power_generation(&rotation, &sun_pos_y), 0.0);
        assert_eq!(panel_pos_z.power_generation(&rotation, &sun_neg_y), 0.0);
        assert_eq!(panel_pos_z.power_generation(&rotation, &sun_pos_z), 0.0);
        assert_eq!(panel_pos_z.power_generation(&rotation, &sun_neg_z), 1.0);

        assert_eq!(panel_neg_z.power_generation(&rotation, &sun_pos_x), 0.0);
        assert_eq!(panel_neg_z.power_generation(&rotation, &sun_neg_x), 0.0);
        assert_eq!(panel_neg_z.power_generation(&rotation, &sun_pos_y), 0.0);
        assert_eq!(panel_neg_z.power_generation(&rotation, &sun_neg_y), 0.0);
        assert_eq!(panel_neg_z.power_generation(&rotation, &sun_pos_z), 1.0);
        assert_eq!(panel_neg_z.power_generation(&rotation, &sun_neg_z), 0.0);
    }

    #[test]
    fn eps() {
        let mut eps = Eps::new(-1.0, 20.0);
        assert_eq!(eps.consumption, -1.0);
        assert_eq!(eps.charge, 20.0);
        assert_eq!(eps.max_charge, 20.0);

        let timestep = 1.0;
        eps.update_capacity(1.0, timestep);
        assert_eq!(eps.charge, eps.max_charge);

        eps.update_capacity(-1.0, timestep);
        assert_eq!(
            eps.charge,
            (-1.0 * timestep + (eps.max_charge * time::HOUR)) / time::HOUR
        );
    }

    #[test]
    fn component() {
        let comp = Component::new("ADCS", -1.0);
        assert_eq!(comp.name, "ADCS".to_string());
        assert_eq!(comp.consumption, -1.0);
    }
}
