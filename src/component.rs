use crate::vector;

#[derive(Debug, PartialEq)]
pub struct SolarPanel {
    pub orientation: vector::Vector3,
    pub power_generation: f64, // [W]
}

impl SolarPanel {
    pub fn new(power_generation: f64, orientation: (f64, f64, f64)) -> Self {
        SolarPanel {
            orientation: vector::Vector3::new(orientation),
            power_generation,
        }
    }

    pub fn power_generation(&self, sun: &vector::Vector3) -> f64 {
        // Angle
        let angle = self.orientation.negative().angle_to(sun);

        // match angle, with cosine
        match angle >= std::f64::consts::FRAC_PI_2 {
            true => 0.0,
            false => self.power_generation * angle.cos(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_new() {
        let panel_zero = SolarPanel::new(0.0, (0.0, 0.0, 0.0));
        let panel_x = SolarPanel::new(0.0, (1.0, 0.0, 0.0));
        let panel_y = SolarPanel::new(0.0, (0.0, 1.0, 0.0));
        let panel_z = SolarPanel::new(0.0, (0.0, 0.0, 1.0));
        assert_eq!(
            panel_zero.orientation,
            vector::Vector3::new((0.0, 0.0, 0.0))
        );
        assert_eq!(panel_x.orientation, vector::Vector3::new((1.0, 0.0, 0.0)));
        assert_eq!(panel_y.orientation, vector::Vector3::new((0.0, 1.0, 0.0)));
        assert_eq!(panel_z.orientation, vector::Vector3::new((0.0, 0.0, 1.0)));
    }

    #[test]
    fn power_generation() {
        let panel_pos_x = SolarPanel::new(1.0, (1.0, 0.0, 0.0));
        let panel_neg_x = SolarPanel::new(1.0, (-1.0, 0.0, 0.0));
        let panel_pos_y = SolarPanel::new(1.0, (0.0, 1.0, 0.0));
        let panel_neg_y = SolarPanel::new(1.0, (0.0, -1.0, 0.0));
        let panel_pos_z = SolarPanel::new(1.0, (0.0, 0.0, 1.0));
        let panel_neg_z = SolarPanel::new(1.0, (0.0, 0.0, -1.0));
        let sun_pos_x = vector::Vector3::new((1.0, 0.0, 0.0));
        let sun_neg_x = vector::Vector3::new((-1.0, 0.0, 0.0));
        let sun_pos_y = vector::Vector3::new((0.0, 1.0, 0.0));
        let sun_neg_y = vector::Vector3::new((0.0, -1.0, 0.0));
        let sun_pos_z = vector::Vector3::new((0.0, 0.0, 1.0));
        let sun_neg_z = vector::Vector3::new((0.0, 0.0, -1.0));

        assert_eq!(panel_pos_x.power_generation(&sun_pos_x), 0.0);
        assert_eq!(panel_pos_x.power_generation(&sun_neg_x), 1.0);
        assert_eq!(panel_pos_x.power_generation(&sun_pos_y), 0.0);
        assert_eq!(panel_pos_x.power_generation(&sun_neg_y), 0.0);
        assert_eq!(panel_pos_x.power_generation(&sun_pos_z), 0.0);
        assert_eq!(panel_pos_x.power_generation(&sun_neg_z), 0.0);

        assert_eq!(panel_neg_x.power_generation(&sun_pos_x), 1.0);
        assert_eq!(panel_neg_x.power_generation(&sun_neg_x), 0.0);
        assert_eq!(panel_neg_x.power_generation(&sun_pos_y), 0.0);
        assert_eq!(panel_neg_x.power_generation(&sun_neg_y), 0.0);
        assert_eq!(panel_neg_x.power_generation(&sun_pos_z), 0.0);
        assert_eq!(panel_neg_x.power_generation(&sun_neg_z), 0.0);

        assert_eq!(panel_pos_y.power_generation(&sun_pos_x), 0.0);
        assert_eq!(panel_pos_y.power_generation(&sun_neg_x), 0.0);
        assert_eq!(panel_pos_y.power_generation(&sun_pos_y), 0.0);
        assert_eq!(panel_pos_y.power_generation(&sun_neg_y), 1.0);
        assert_eq!(panel_pos_y.power_generation(&sun_pos_z), 0.0);
        assert_eq!(panel_pos_y.power_generation(&sun_neg_z), 0.0);

        assert_eq!(panel_neg_y.power_generation(&sun_pos_x), 0.0);
        assert_eq!(panel_neg_y.power_generation(&sun_neg_x), 0.0);
        assert_eq!(panel_neg_y.power_generation(&sun_pos_y), 1.0);
        assert_eq!(panel_neg_y.power_generation(&sun_neg_y), 0.0);
        assert_eq!(panel_neg_y.power_generation(&sun_pos_z), 0.0);
        assert_eq!(panel_neg_y.power_generation(&sun_neg_z), 0.0);

        assert_eq!(panel_pos_z.power_generation(&sun_pos_x), 0.0);
        assert_eq!(panel_pos_z.power_generation(&sun_neg_x), 0.0);
        assert_eq!(panel_pos_z.power_generation(&sun_pos_y), 0.0);
        assert_eq!(panel_pos_z.power_generation(&sun_neg_y), 0.0);
        assert_eq!(panel_pos_z.power_generation(&sun_pos_z), 0.0);
        assert_eq!(panel_pos_z.power_generation(&sun_neg_z), 1.0);

        assert_eq!(panel_neg_z.power_generation(&sun_pos_x), 0.0);
        assert_eq!(panel_neg_z.power_generation(&sun_neg_x), 0.0);
        assert_eq!(panel_neg_z.power_generation(&sun_pos_y), 0.0);
        assert_eq!(panel_neg_z.power_generation(&sun_neg_y), 0.0);
        assert_eq!(panel_neg_z.power_generation(&sun_pos_z), 1.0);
        assert_eq!(panel_neg_z.power_generation(&sun_neg_z), 0.0);
    }
}
