#[path = "./tests/orbit.rs"]
mod tests;

use crate::cubesat;
use crate::vector;

static CONST_G: f64 = 6.6743015e-11; // [N*m^2*kg^-2]
static MASS_EARTH: f64 = 5.9722e24; // [kg]
static CONST_MU: f64 = CONST_G * MASS_EARTH;
pub static RADIUS_EARTH: f64 = 6.3781e6; // [m]

#[derive(Debug, PartialEq)]
pub enum OrbitType {
    CircularCosine,
    Parametric,
}

#[derive(Debug, PartialEq)]
pub struct OrbitParameters {
    pub radius: Option<f64>,                      // [m]
    pub inclination: Option<f64>,                 // [deg]
    pub argument_of_periapsis: Option<f64>,       // [deg]
    pub longitude_of_ascending_node: Option<f64>, // [deg]
    pub semi_major_axis: Option<f64>,             // [m]
    pub eccentricity: Option<f64>,
}

impl OrbitParameters {
    pub fn new() -> Self {
        OrbitParameters {
            radius: None,
            inclination: None,
            argument_of_periapsis: None,
            longitude_of_ascending_node: None,
            semi_major_axis: None,
            eccentricity: None,
        }
    }

    pub fn set_altitude(&mut self, altitude: f64) {
        self.radius = Some(RADIUS_EARTH + altitude);
    }

    pub fn set_inclination(&mut self, inclination: f64) {
        self.inclination = Some(inclination);
    }

    pub fn set_argument_of_periapsis(&mut self, argument_of_periapsis: f64) {
        self.argument_of_periapsis = Some(argument_of_periapsis);
    }

    pub fn set_longitude_of_ascending_node(&mut self, longitude_of_ascending_node: f64) {
        self.longitude_of_ascending_node = Some(longitude_of_ascending_node);
    }

    pub fn set_semi_major_axis(&mut self, semi_major_axis: f64) {
        self.semi_major_axis = Some(semi_major_axis);
    }

    pub fn set_eccentricity(&mut self, eccentricity: f64) {
        self.eccentricity = Some(eccentricity);
    }
}

pub fn orbit_circular_cosine(cubesat: &mut cubesat::CubeSat) {
    // Guards and values
    let pos = cubesat.pos.as_mut().expect("No position vector is set!");
    let vel = cubesat.vel.as_mut().expect("No velocity vector is set!");
    let parameters = cubesat
        .orbit_parameters
        .as_ref()
        .expect("No orbit parameters are set!");
    let alt = parameters.radius.as_ref().expect("No altitude is set!");
    let inc = parameters
        .inclination
        .as_ref()
        .expect("No inclination is set!");
    let ap = parameters
        .argument_of_periapsis
        .as_ref()
        .expect("No argument of periapsis is set!");
    let lan = parameters
        .longitude_of_ascending_node
        .as_ref()
        .expect("No longitude of ascending node is set!");
    let acc = cubesat
        .acc
        .as_mut()
        .expect("No acceleration vector is set!");
    let time = cubesat.time.as_ref().expect("No time is set!");
    let omega = (alt.powi(3) / (CONST_G * MASS_EARTH)).powf(-0.5);
    let ang_to_rad = std::f64::consts::PI / 180.0;

    // Semi-major axis = radius
    // pos_x(t) = r * cos([r^3/GM]^-(1/2)*t)
    // pos_y(t) = r * sin([r^3/GM]^-(1/2)*t)

    // Calculate new vectors
    // x = r * cos(wt)
    // y = r * sin(wt)
    *pos = vector::Vector3::new(
        alt * (omega * time.now).cos(),
        alt * (omega * time.now).sin(),
        0.0,
    )
    .rot_y(*inc * ang_to_rad)
    .rot_z(*ap * ang_to_rad);

    // x' = -w * r * sin(wt)
    // y' =  w * r * cos(wt)
    *vel = vector::Vector3::new(
        -omega * alt * (omega * time.now).sin(),
        omega * alt * (omega * time.now).cos(),
        0.0,
    )
    .rot_z(*ap * ang_to_rad)
    .rot_y(*inc * ang_to_rad)
    .rot_z(*lan * ang_to_rad);

    // x'' = -w^2 * r * cos(wt)
    // y'' = -w^2 * r * sin(wt)
    *acc = vector::Vector3::new(
        -omega * omega * alt * (omega * time.now).cos(),
        -omega * omega * alt * (omega * time.now).sin(),
        0.0,
    )
    .rot_z(*ap * ang_to_rad)
    .rot_y(*inc * ang_to_rad)
    .rot_z(*lan * ang_to_rad);
}

#[allow(non_snake_case)]
pub fn orbit_parametric(cubesat: &mut cubesat::CubeSat) {
    // Guards and values
    let parameters = cubesat
        .orbit_parameters
        .as_ref()
        .expect("No orbit parameters are set!");
    let ecc = parameters
        .eccentricity
        .as_ref()
        .expect("No eccentricity is set!");
    if ecc == &0.0 {
        panic!("Parametric orbit not possible with e = 0.0!");
    }
    let t = cubesat.time.as_ref().expect("No time is set!").now;
    let pos = cubesat.pos.as_mut().expect("No position vector is set!");
    let vel = cubesat.vel.as_mut().expect("No velocity vector is set!");
    let semi = parameters
        .semi_major_axis
        .as_ref()
        .expect("No semi-major axis is set!");
    let inc = parameters
        .inclination
        .as_ref()
        .expect("No inclination is set!");
    let ap = parameters
        .argument_of_periapsis
        .as_ref()
        .expect("No argument of periapsis is set!");
    let lan = parameters
        .longitude_of_ascending_node
        .as_ref()
        .expect("No longitude of ascending node is set!");
    let PI = std::f64::consts::PI;
    let ang_to_rad = PI / 180.0;

    // 1. Find E using the Bisect method
    let n = (CONST_MU / semi.powi(3)).sqrt();
    let T = 2.0 * PI / n;
    let t = t.rem_euclid(T); // Time in orbit
    let f = |x: f64| x - ecc * x.sin() - n * t;
    let mut E = 0.0;
    let mut a = 0.0 - 0.1;
    let mut b = 2.0 * PI + 0.1;
    let epsilon = 2.0 * PI / (T * 100.0);
    let max_iter = 20;
    for _ in 0..max_iter {
        let c = (a + b) / 2.0;
        E = c;
        let fc = f(c);
        if fc.abs() <= epsilon {
            break;
        }

        let fa = f(a);
        if fa * fc < 0.0 {
            b = c;
        } else {
            a = c;
        }
    }

    // 2. Calculate the true anomaly v
    let beta = ecc / (1.0 + (1.0 - ecc.powi(2)).sqrt());
    let v = E + 2.0 * (beta * E.sin() / (ecc - beta * E.cos())).atan();

    // 3. Calculate position vector using v, p, e
    let p = semi * (1.0 - ecc.powi(2));
    *pos = vector::Vector3::new(
        p * v.cos() / (1.0 + ecc * v.cos()),
        p * v.sin() / (1.0 + ecc * v.cos()),
        0.0,
    )
    // 3.5 Rotate position vector by inclination, argument of periapsis, longitude of ascending node
    .rot_z(*ap * ang_to_rad)
    .rot_y(*inc * ang_to_rad)
    .rot_z(*lan * ang_to_rad);

    // 4. Calculate velocity vector using v, µ, p, e
    *vel = vector::Vector3::new(
        -(CONST_MU / p).sqrt() * v.sin(),
        (CONST_MU / p).sqrt() * (ecc + v.cos()),
        0.0,
    )
    // 4.5 Rotate velocity vector by inclination, argument of periapsis, longitude of ascending node
    .rot_z(*ap * ang_to_rad)
    .rot_y(*inc * ang_to_rad)
    .rot_z(*lan * ang_to_rad);

    // (Acceleration?)
}
