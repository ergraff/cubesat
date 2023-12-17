mod component;
mod cubesat;
mod orbit;
mod time;
mod vector;

use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("./input/");
    let files = fs::read_dir(path).expect("Unable to read input path!");

    for file in files {
        let f = file.expect("Unable to read file!").path();
        println!("Simulating: {}", f.display());

        let mut cubesat = cubesat::CubeSat::from_toml(f.to_str().unwrap());
        cubesat.simulate();
    }
    println!("Simulation ended");
}
