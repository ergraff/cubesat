use crate::cubesat;
use crate::vector;

static CONST_G: f64 = 6.6743015e-11; // [N*m^2*kg^-2]
static MASS_EARTH: f64 = 5.9722e24; // [kg]
pub static RADIUS_EARTH: f64 = 6.3781e6; // [m]

#[derive(Debug, PartialEq)]
pub enum OrbitType {
    EquatorialCosine,
    CircularCosine,
}

#[derive(Debug, PartialEq)]
pub struct OrbitParameters {
    // Equatorial and circular
    pub radius: Option<f64>,                // [m]
    pub inclination: Option<f64>,           // [deg]
    pub argument_of_periapsis: Option<f64>, // [deg]
}

impl OrbitParameters {
    pub fn new() -> Self {
        OrbitParameters {
            radius: None,
            inclination: None,
            argument_of_periapsis: None,
        }
    }

    pub fn set_radius(&mut self, radius: f64) {
        self.radius = Some(RADIUS_EARTH + radius);
    }

    pub fn set_inclination(&mut self, inclination: f64) {
        self.inclination = Some(inclination);
    }

    pub fn set_argument_of_periapsis(&mut self, argument_of_periapsis: f64) {
        self.argument_of_periapsis = Some(argument_of_periapsis);
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
    let pos = cubesat.pos.as_mut().expect("No position vector is set!");
    let vel = cubesat.vel.as_mut().expect("No velocity vector is set!");
    let acc = cubesat
        .acc
        .as_mut()
        .expect("No acceleration vector is set!");
    let time = cubesat.time.as_ref().expect("No time is set!");
    let omega = (r.powi(3) / (CONST_G * MASS_EARTH)).powf(-0.5);

    // Semi-major axis = radius r
    // pos_x(t) = r * cos([r^3/GM]^-(1/2)*t)
    // pos_y(t) = r * sin([r^3/GM]^-(1/2)*t)

    // Calculate new vectors
    // x = r * cos(wt)
    // y = r * sin(wt)
    *pos = vector::Vector3::new(
        r * (omega * time.now).cos(),
        r * (omega * time.now).sin(),
        0.0,
    );

    // x' = -w * r * sin(wt)
    // y' = w * r * cos(wt)
    *vel = vector::Vector3::new(
        -omega * r * (omega * time.now).sin(),
        omega * r * (omega * time.now).cos(),
        0.0,
    );

    // x'' = -w^2 * r * cos(wt)
    // y'' = -w^2 * r * sin(wt)
    *acc = vector::Vector3::new(
        -omega * omega * r * (omega * time.now).cos(),
        -omega * omega * r * (omega * time.now).sin(),
        0.0,
    );
}

pub fn orbit_circular_cosine(cubesat: &mut cubesat::CubeSat) {
    // Guards and values
    let parameters = cubesat
        .orbit_parameters
        .as_ref()
        .expect("No orbit parameters are set!");
    let r = parameters.radius.as_ref().expect("No radius is set!");
    let inc = parameters
        .inclination
        .as_ref()
        .expect("No inclination is set!");
    let ap = parameters
        .argument_of_periapsis
        .as_ref()
        .expect("No argument of periapsis is set!");
    let pos = cubesat.pos.as_mut().expect("No position vector is set!");
    let vel = cubesat.vel.as_mut().expect("No velocity vector is set!");
    let acc = cubesat
        .acc
        .as_mut()
        .expect("No acceleration vector is set!");
    let time = cubesat.time.as_ref().expect("No time is set!");
    let omega = (r.powi(3) / (CONST_G * MASS_EARTH)).powf(-0.5);
    let ang_to_rad = std::f64::consts::PI / 180.0;

    // Semi-major axis = radius
    // pos_x(t) = r * cos([r^3/GM]^-(1/2)*t)
    // pos_y(t) = r * sin([r^3/GM]^-(1/2)*t)

    // Calculate new vectors
    // x = r * cos(wt)
    // y = r * sin(wt)
    *pos = vector::Vector3::new(
        r * (omega * time.now).cos(),
        r * (omega * time.now).sin(),
        0.0,
    )
    .rot_y(*inc * ang_to_rad)
    .rot_z(*ap * ang_to_rad);

    // x' = -w * r * sin(wt)
    // y' =  w * r * cos(wt)
    *vel = vector::Vector3::new(
        -omega * r * (omega * time.now).sin(),
        omega * r * (omega * time.now).cos(),
        0.0,
    )
    .rot_y(*inc * ang_to_rad)
    .rot_z(*ap * ang_to_rad);

    // x'' = -w^2 * r * cos(wt)
    // y'' = -w^2 * r * sin(wt)
    *acc = vector::Vector3::new(
        -omega * omega * r * (omega * time.now).cos(),
        -omega * omega * r * (omega * time.now).sin(),
        0.0,
    )
    .rot_y(*inc * ang_to_rad)
    .rot_z(*ap * ang_to_rad);
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

    #[test]
    fn orbit_equatorial_cosine() {
        let mut cubesat = cubesat::CubeSat::new()
            .with_orbit_type("equatorial cosine")
            .with_orbit_parameters(vec![("radius", 500_000.0)])
            .with_position(0.0, 0.0, 0.0)
            .with_velocity(0.0, 0.0, 0.0)
            .with_acceleration(0.0, 0.0, 0.0)
            .with_time(0.0, 1.0, 1.0);

        // cubesat.update_orbit();
        super::orbit_equatorial_cosine(&mut cubesat);
        let pos = cubesat.pos.unwrap();
        let vel = cubesat.vel.unwrap();
        let acc = cubesat.acc.unwrap();

        assert_eq!(pos.x, RADIUS_EARTH + 500_000.0);
        assert_eq!(pos.y, 0.0);
        assert_eq!(pos.z, 0.0);

        assert!(vel.x == 0.0);
        assert!(vel.y > 0.0);
        assert!(vel.z == 0.0);

        assert!(acc.x < 0.0);
        assert!(acc.y == 0.0);
        assert!(acc.z == 0.0);
    }

    #[test]
    fn orbit_circular_cosine() {
        // Equatorial
        let mut cubesat = cubesat::CubeSat::new()
            .with_orbit_type("circular cosine")
            .with_orbit_parameters(vec![
                ("radius", 500_000.0),
                ("inclination", 0.0),
                ("argument of periapsis", 0.0),
            ])
            .with_position(0.0, 0.0, 0.0)
            .with_velocity(0.0, 0.0, 0.0)
            .with_acceleration(0.0, 0.0, 0.0)
            .with_time(0.0, 1.0, 1.0);

        // cubesat.update_orbit();
        super::orbit_circular_cosine(&mut cubesat);
        let pos = cubesat.pos.unwrap();
        let vel = cubesat.vel.unwrap();
        let acc = cubesat.acc.unwrap();

        assert_eq!(pos.x, RADIUS_EARTH + 500_000.0);
        assert_eq!(pos.y, 0.0);
        assert_eq!(pos.z, 0.0);

        assert_eq!(vel.x, 0.0);
        assert!(vel.y > 0.0);
        assert_eq!(vel.z, 0.0);

        assert!(acc.x < 0.0);
        assert_eq!(acc.y, 0.0);
        assert_eq!(acc.z, 0.0);

        // Polar
        let mut cubesat = cubesat::CubeSat::new()
            .with_orbit_type("circular cosine")
            .with_orbit_parameters(vec![
                ("radius", 500_000.0),
                ("inclination", 90.0),
                ("argument of periapsis", 0.0),
            ])
            .with_position(0.0, 0.0, 0.0)
            .with_velocity(0.0, 0.0, 0.0)
            .with_acceleration(0.0, 0.0, 0.0)
            .with_time(0.0, 1.0, 1.0);

        // cubesat.update_orbit();
        super::orbit_circular_cosine(&mut cubesat);
        let pos = cubesat.pos.unwrap();
        let vel = cubesat.vel.unwrap();
        let acc = cubesat.acc.unwrap();

        assert!(pos.x.abs() < 0.0001); // Float rounding error, very close to 0
        assert_eq!(pos.y, 0.0);
        assert_eq!(pos.z, -(RADIUS_EARTH + 500_000.0));

        assert_eq!(vel.x, 0.0);
        assert!(vel.y > 0.0);
        assert_eq!(vel.z, 0.0);

        assert!(acc.x.abs() < 0.0001); // Float rounding error, very close to 0
        assert_eq!(acc.y, 0.0);
        assert!(acc.z > 0.0);
    }
}
