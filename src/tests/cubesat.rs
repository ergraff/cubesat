#[allow(unused_imports)]
use crate::cubesat::*;

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
    let cubesat = CubeSat::new().with_orbit_type("circular cosine");
    assert_ne!(cubesat.orbit_type, None);
    assert_eq!(
        cubesat.orbit_type.unwrap(),
        orbit::OrbitType::CircularCosine
    );
}

#[test]
fn with_orbit_parameters() {
    let cubesat = CubeSat::new().with_orbit_parameters(vec![("altitude", 500_000.0)]);
    assert_ne!(cubesat.orbit_parameters, None);
    assert_eq!(
        cubesat.orbit_parameters.as_ref().unwrap().radius.unwrap(),
        orbit::RADIUS_EARTH + 500_000.0
    );
}

#[test]
fn with_position() {
    let cubesat_x = CubeSat::new().with_position(1.0, 0.0, 0.0);
    let cubesat_y = CubeSat::new().with_position(0.0, 1.0, 0.0);
    let cubesat_z = CubeSat::new().with_position(0.0, 0.0, 1.0);
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
    let cubesat_x = CubeSat::new().with_velocity(1.0, 0.0, 0.0);
    let cubesat_y = CubeSat::new().with_velocity(0.0, 1.0, 0.0);
    let cubesat_z = CubeSat::new().with_velocity(0.0, 0.0, 1.0);
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
    let cubesat_x = CubeSat::new().with_acceleration(1.0, 0.0, 0.0);
    let cubesat_y = CubeSat::new().with_acceleration(0.0, 1.0, 0.0);
    let cubesat_z = CubeSat::new().with_acceleration(0.0, 0.0, 1.0);
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
    let cubesat_x = CubeSat::new().with_rotation(1.0, 0.0, 0.0);
    let cubesat_y = CubeSat::new().with_rotation(0.0, 1.0, 0.0);
    let cubesat_z = CubeSat::new().with_rotation(0.0, 0.0, 1.0);
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
fn with_sun() {
    let cubesat_x = CubeSat::new().with_sun(1.0, 0.0, 0.0);
    let cubesat_y = CubeSat::new().with_sun(0.0, 1.0, 0.0);
    let cubesat_z = CubeSat::new().with_sun(0.0, 0.0, 1.0);
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
fn with_eps() {
    let cubesat = CubeSat::new().with_eps(-1.0, 20.0);
    assert_ne!(cubesat.eps, None);
    assert_eq!(cubesat.eps.as_ref().unwrap().consumption, -1.0);
    assert_eq!(cubesat.eps.as_ref().unwrap().charge, 20.0);
    assert_eq!(cubesat.eps.as_ref().unwrap().max_charge, 20.0);
}

#[test]
fn with_component() {
    let cubesat = CubeSat::new().with_component("ADCS", -1.0);
    assert_ne!(cubesat.components, None);
    assert_eq!(
        cubesat.components.as_ref().unwrap()[0].name,
        "ADCS".to_string()
    );
    assert_eq!(cubesat.components.as_ref().unwrap()[0].consumption, -1.0);
}

#[test]
fn in_eclipse() {
    // In the sun
    let radius = orbit::RADIUS_EARTH + 500_000.0;
    let cubesat_x = CubeSat::new()
        .with_sun(-1.0, 0.0, 0.0)
        .with_position(radius, 0.0, 0.0);
    let cubesat_y = CubeSat::new()
        .with_sun(-1.0, 0.0, 0.0)
        .with_position(0.0, radius, 0.0);
    let cubesat_my = CubeSat::new()
        .with_sun(-1.0, 0.0, 0.0)
        .with_position(0.0, -radius, 0.0);
    let cubesat_z = CubeSat::new()
        .with_sun(-1.0, 0.0, 0.0)
        .with_position(0.0, 0.0, radius);
    let cubesat_mz = CubeSat::new()
        .with_sun(-1.0, 0.0, 0.0)
        .with_position(0.0, 0.0, -radius);
    assert_eq!(cubesat_x.in_eclipse(), false);
    assert_eq!(cubesat_y.in_eclipse(), false);
    assert_eq!(cubesat_my.in_eclipse(), false);
    assert_eq!(cubesat_z.in_eclipse(), false);
    assert_eq!(cubesat_mz.in_eclipse(), false);

    // In the eclipse
    let cubesat_center = CubeSat::new()
        .with_sun(-1.0, 0.0, 0.0)
        .with_position(-radius, 0.0, 0.0);
    let cubesat_center_y =
        CubeSat::new()
            .with_sun(-1.0, 0.0, 0.0)
            .with_position(-radius, orbit::RADIUS_EARTH, 0.0);
    let cubesat_center_my =
        CubeSat::new()
            .with_sun(-1.0, 0.0, 0.0)
            .with_position(-radius, -orbit::RADIUS_EARTH, 0.0);
    let cubesat_center_z =
        CubeSat::new()
            .with_sun(-1.0, 0.0, 0.0)
            .with_position(-radius, 0.0, orbit::RADIUS_EARTH);
    let cubesat_center_mz =
        CubeSat::new()
            .with_sun(-1.0, 0.0, 0.0)
            .with_position(-radius, 0.0, -orbit::RADIUS_EARTH);
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
        .with_position(orbit::RADIUS_EARTH + 500_000.0, 0.0, 0.0)
        .with_rotation(0.0, 0.0, 0.0)
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
    let cubesat_x = cubesat.with_sun(1.0, 0.0, 0.0);
    assert_eq!(cubesat_x.get_power_generation(), 0.0);

    let cubesat_mx = cubesat_x.with_sun(-1.0, 0.0, 0.0);
    assert_eq!(cubesat_mx.get_power_generation(), 1.0);

    let cubesat_y = cubesat_mx.with_sun(0.0, 1.0, 0.0);
    assert_eq!(cubesat_y.get_power_generation(), 1.0);

    let cubesat_my = cubesat_y.with_sun(0.0, -1.0, 0.0);
    assert_eq!(cubesat_my.get_power_generation(), 1.0);

    let cubesat_z = cubesat_my.with_sun(0.0, 0.0, 1.0);
    assert_eq!(cubesat_z.get_power_generation(), 1.0);

    let cubesat_mz = cubesat_z.with_sun(0.0, 0.0, -1.0);
    assert_eq!(cubesat_mz.get_power_generation(), 1.0);
}

#[test]
fn iterate() {
    let mut cubesat = CubeSat::new().with_time(0.0, 10.0, 1.0);
    for _ in 0..11 {
        assert_eq!(cubesat.active, true);
        cubesat.iterate();
    }
    assert_eq!(cubesat.active, false);
}

#[test]
fn update_rotation() {
    let mut cubesat_pos = CubeSat::new()
        .with_time(0.0, 10.0, 1.0)
        .with_rotation(0.0, 0.0, 0.0)
        .with_rotation_velocity(0.0, 0.0, 0.0)
        .with_rotation_acceleration(1.0, 1.0, 1.0);
    let mut cubesat_neg = CubeSat::new()
        .with_time(0.0, 10.0, 1.0)
        .with_rotation(0.0, 0.0, 0.0)
        .with_rotation_velocity(0.0, 0.0, 0.0)
        .with_rotation_acceleration(-1.0, -1.0, -1.0);

    for _ in 0..10 {
        cubesat_pos.update_rotation();
        cubesat_neg.update_rotation();
    }

    assert_eq!(
        cubesat_pos.rot,
        Some(vector::Vector3::new(45.0, 45.0, 45.0))
    );
    assert_eq!(
        cubesat_neg.rot,
        Some(vector::Vector3::new(-45.0, -45.0, -45.0))
    );
    assert_eq!(
        cubesat_pos.rot_vel,
        Some(vector::Vector3::new(10.0, 10.0, 10.0))
    );
    assert_eq!(
        cubesat_neg.rot_vel,
        Some(vector::Vector3::new(-10.0, -10.0, -10.0))
    );
    assert_eq!(
        cubesat_pos.rot_acc,
        Some(vector::Vector3::new(1.0, 1.0, 1.0))
    );
    assert_eq!(
        cubesat_neg.rot_acc,
        Some(vector::Vector3::new(-1.0, -1.0, -1.0))
    );
}

#[test]
fn rotate_sun() {
    let mut cubesat = CubeSat::new()
        .with_time(0.0, 0.0, 1.0)
        .with_sun(1.0, 0.0, 0.0);

    // 0 months in
    let sun = cubesat.sun.unwrap();
    assert_eq!(sun.x, 1.0);
    assert_eq!(sun.y, 0.0);
    assert_eq!(sun.z, 0.0);

    // 3 month in
    let time = 90 * time::DAY as usize;
    for _ in 0..time {
        cubesat.rotate_sun();
    }
    let sun = cubesat.sun.unwrap();
    assert!(0.0 < sun.y);
    assert_eq!(sun.z, 0.0);

    // 6 months in
    let time = 90 * time::DAY as usize;
    for _ in 0..time {
        cubesat.rotate_sun();
    }
    let sun = cubesat.sun.unwrap();
    assert!(sun.x < 0.0);
    assert_eq!(sun.z, 0.0);

    // 9 months in
    let time = 90 * time::DAY as usize;
    for _ in 0..time {
        cubesat.rotate_sun();
    }
    let sun = cubesat.sun.unwrap();
    assert!(sun.y < 0.0);
    assert_eq!(sun.z, 0.0);

    // 12 months in
    let time = 183 * time::DAY as usize;
    for _ in 0..time {
        cubesat.rotate_sun();
    }
    let sun = cubesat.sun.unwrap();
    assert!(0.0 < sun.x);
    assert_eq!(sun.z, 0.0);
}

#[test]
fn history_new() {
    let history = History::new();
    assert!(history.time.is_empty());
    assert!(history.pos.is_empty());
    assert!(history.vel.is_empty());
    assert!(history.acc.is_empty());
    assert!(history.rot.is_empty());
    assert!(history.sun.is_empty());
    assert!(history.charge.is_empty());
}

#[test]
fn history_save() {
    let mut cubesat = CubeSat::new()
        .with_time(0.0, 1.0, 1.0)
        .with_position(1.0, 1.0, 1.0)
        .with_velocity(1.0, 1.0, 1.0)
        .with_acceleration(1.0, 1.0, 1.0)
        .with_rotation(1.0, 1.0, 1.0)
        .with_sun(1.0, 1.0, 1.0)
        .with_eps(-1.0, 1.0);
    cubesat.save_history();
    cubesat.iterate();
    cubesat.save_history();

    let history = cubesat.history;

    // First step
    assert_eq!(history.time[0], 0.0);
    assert_eq!(history.pos[0], (1.0, 1.0, 1.0));
    assert_eq!(history.vel[0], (1.0, 1.0, 1.0));
    assert_eq!(history.acc[0], (1.0, 1.0, 1.0));
    assert_eq!(history.rot[0], (1.0, 1.0, 1.0));
    assert_eq!(history.sun[0], (1.0, 1.0, 1.0));
    assert_eq!(history.charge[0], 1.0);

    // Second step
    assert_eq!(history.time[1], 1.0);
    assert_eq!(history.pos[1], (1.0, 1.0, 1.0));
    assert_eq!(history.vel[1], (1.0, 1.0, 1.0));
    assert_eq!(history.acc[1], (1.0, 1.0, 1.0));
    assert_eq!(history.rot[1], (1.0, 1.0, 1.0));
    assert_eq!(history.sun[1], (1.0, 1.0, 1.0));
    assert_eq!(history.charge[1], 1.0);
}
