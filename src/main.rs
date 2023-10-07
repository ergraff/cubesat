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

    let cubesat = cubesat::CubeSat::new("APTAS", 500.0, (0.0, 100.0, 1.0));
}
