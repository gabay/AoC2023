use std::fs;
use std::path::Path;

// Common functionality for AoC

pub fn hello_world() {
    println!("Hello, world!");
}

pub fn readfile(path: &str) -> String {
    fs::read_to_string(Path::new(path))
        .unwrap()
        .trim_matches('\n')
        .to_string()
}

pub fn to_i32(v: &str) -> i32 {
    v.parse::<i32>().unwrap()
}
