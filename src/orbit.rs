use crate::cubesat;
use crate::vector;

static CONST_G: f64 = 6.6743015e-11; // [N*m^2*kg^-2]
static MASS_EARTH: f64 = 5.9722e24; // [kg]
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

pub fn orbit_equatorial_cosine(cubesat: &mut cubesat::CubeSat) {
    // Guards and values
    let r = cubesat
        .orbit_parameters
        .as_ref()
        .expect("No orbit parameters are set!")
        .radius
        .as_ref()
        .expect("No radius is set!");
    let pos = &mut cubesat.pos.expect("No position vector is set!");
    let vel = &mut cubesat.vel.expect("No velocity vector is set");
    let acc = &mut cubesat.acc.expect("No acceleration vector is set!");
    let time = cubesat.time.as_ref().expect("No time is set!");
    let omega = (r.powi(3) / (CONST_G * MASS_EARTH)).powf(-0.5);

    // Semi-major axis = radius r
    // pos_x(t) = r * cos([r^3/GM]^-(1/2)*t)
    // pos_y(t) = r * sin([r^3/GM]^-(1/2)*t)

    // Calculate new vectors
    // x = r * cos(wt)
    // y = r * sin(wt)
    *pos = vector::Vector3::new((
        r * (omega * time.now).cos(),
        r * (omega * time.now).sin(),
        0.0,
    ));

    // x' = -w * r * sin(wt)
    // y' = w * r * cos(wt)
    *vel = vector::Vector3::new((
        -omega * r * (omega * time.now).sin(),
        omega * r * (omega * time.now).cos(),
        0.0,
    ));

    // x'' = -w^2 * r * cos(wt)
    // y'' = -w^2 * r * sin(wt)
    *acc = vector::Vector3::new((
        -omega * omega * r * (omega * time.now).cos(),
        -omega * omega * r * (omega * time.now).sin(),
        0.0,
    ));
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
