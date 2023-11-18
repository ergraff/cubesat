#[allow(unused_imports)]
use crate::orbit::*;

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

#[test]
fn orbit_parametric() {
    // Equatorial
    let mut cubesat = cubesat::CubeSat::new()
        .with_orbit_type("circular cosine")
        .with_orbit_parameters(vec![
            ("semi-major axis", 500_000.0),
            ("inclination", 0.0),
            ("argument of periapsis", 0.0),
            ("eccentricity", 0.001),
        ])
        .with_position(0.0, 0.0, 0.0)
        .with_velocity(0.0, 0.0, 0.0)
        .with_acceleration(0.0, 0.0, 0.0)
        .with_time(0.0, 1.0, 1.0);

    // cubesat.update_orbit();
    super::orbit_parametric(&mut cubesat);
    let pos = cubesat.pos.unwrap();
    let vel = cubesat.vel.unwrap();
    let acc = cubesat.acc.unwrap();

    assert!(pos.x != 0.0);
    assert!(pos.y != 0.0);
    assert!(pos.z == 0.0);

    assert!(vel.x != 0.0);
    assert!(vel.y != 0.0);
    assert!(vel.z == 0.0);

    assert!(acc.x == 0.0);
    assert!(acc.y == 0.0);
    assert!(acc.z == 0.0);

    // Polar
    let mut cubesat = cubesat::CubeSat::new()
        .with_orbit_type("circular cosine")
        .with_orbit_parameters(vec![
            ("semi-major axis", 500_000.0),
            ("inclination", 90.0),
            ("argument of periapsis", 45.0),
            ("eccentricity", 0.001),
        ])
        .with_position(0.0, 0.0, 0.0)
        .with_velocity(0.0, 0.0, 0.0)
        .with_acceleration(0.0, 0.0, 0.0)
        .with_time(1.0, 1.0, 1.0);

    // cubesat.update_orbit();
    super::orbit_parametric(&mut cubesat);
    let pos = cubesat.pos.unwrap();
    let vel = cubesat.vel.unwrap();
    let acc = cubesat.acc.unwrap();

    assert!(pos.x != 0.0);
    assert!(pos.y != 0.0);
    assert!(pos.z != 0.0);

    assert!(vel.x != 0.0);
    assert!(vel.y != 0.0);
    assert!(vel.z != 0.0);

    assert!(acc.x == 0.0);
    assert!(acc.y == 0.0);
    assert!(acc.z == 0.0);
}
