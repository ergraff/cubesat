mod component;
mod cubesat;
mod orbit;
mod time;
mod vector;

fn main() {
    let mut cubesat = cubesat::CubeSat::new()
        .with_name("APTAS")
        .with_time(0.0, 86400.0, 1.0)
        .with_orbit_type("circular cosine")
        .with_orbit_parameters(vec![
            ("radius", 500_000.0),
            ("inclination", 45.0),
            ("argument of periapsis", 45.0),
        ])
        .with_position(0.0, 0.0, 0.0)
        .with_velocity(0.0, 0.0, 0.0)
        .with_acceleration(0.0, 0.0, 0.0)
        .with_sun(-1.0, 0.0, 0.0)
        .with_rotation(0.0, 0.0, 0.0)
        .with_rotation_velocity(1.0, 0.5, 0.2)
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
        .with_eps(-1.0, 10.0)
        .with_component("ADCS", -1.0);

    cubesat.print();
    cubesat.simulate();
    println!("Simulation ended");
    cubesat.history.write("history.csv");
}
