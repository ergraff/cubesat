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
        .with_orbit_type("equatorial cosine")
        .with_orbit_parameters(vec![("radius", 500_000.0)]);

    cubesat.print();
}
