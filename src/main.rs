mod component;
mod cubesat;
mod orbit;
mod time;
mod vector;

use std::fs;
use std::path::Path;
use std::thread;

fn main() {
    let path = Path::new("./input/");
    let files = fs::read_dir(path).expect("Unable to read input path!");

    let handle = thread::spawn(|| {
        for file in files {
            let f = file.expect("Unable to read file!").path();
            let name = f.file_name().unwrap().to_str().unwrap();
            println!("Simulating '{}'", name);

            let mut cubesat = cubesat::CubeSat::from_toml(f.to_str().unwrap());
            cubesat.simulate();
        }
    });

    handle.join().unwrap();
}
