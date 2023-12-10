mod component;
mod cubesat;
mod orbit;
mod time;
mod vector;

fn main() {
    let mut cubesat = cubesat::CubeSat::new()
        .with_name("APTAS")
        .with_time(0.0, 10000.0, 1.0)
        .with_orbit_type("parametric")
        .with_orbit_parameters(vec![
            ("inclination", 20.0),
            ("argument of periapsis", 25.0),
            ("longitude of ascending node", 15.0),
            ("semi-major axis", orbit::RADIUS_EARTH + 1_000_000.0),
            ("eccentricity", 0.1),
        ])
        .with_position(0.0, 0.0, 0.0)
        .with_velocity(0.0, 0.0, 0.0)
        .with_acceleration(0.0, 0.0, 0.0)
        .with_sun(-1.0, 0.0, 0.0)
        .with_rotation(0.0, 0.0, 0.0)
        .with_rotation_velocity(0.0, 0.0, 0.0)
        .with_rotation_acceleration(0.0, 0.0, 0.0)
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
        )
        .with_eps(-0.3, 10.0)
        .with_component("ADCS", -0.2, Some(-0.4), Some(100.0), Some(10.0))
        .with_safety_limit(20.0);

    cubesat.print();
    cubesat.simulate();
    println!("Simulation ended");
    // cubesat.history.write("history.csv");
}
