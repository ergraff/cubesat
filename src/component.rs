use crate::vector;

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
}

pub fn foo() {
    println!("Hello from component.rs");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panel_origin() {
        let panel = SolarPanel::new(0.0, (0.0, 0.0, 0.0));
        assert_eq!(panel.orientation, vector::Vector3::new((0.0, 0.0, 0.0)));
    }
}
