mod component;
mod cubesat;
mod orbit;
mod time;
mod vector;

fn main() {
    println!("Hello, world!");
    cubesat::foo();
    vector::foo();
    time::foo();
    component::foo();
    orbit::foo();

    let cubesat = cubesat::CubeSat::new()
        .with_name("APTAS")
        .with_time(0.0, 100.0, 1.0)
        .with_orbit_type("equatorial cosine")
        .with_orbit_parameters(vec![("radius", 500_000.0)])
        .with_position((0.0, 0.0, 0.0))
        .with_velocity((0.0, 0.0, 0.0))
        .with_acceleration((0.0, 0.0, 0.0))
        .with_rotation((0.0, 0.0, 0.0))
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

    cubesat.print();
}
