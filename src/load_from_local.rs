// src/load_from_local.rs
use std::fs;

pub mod parser;

fn main() {
    println!("Hello, world!");

    // Grab JSON file
    let file_path = "data/test.json".to_owned();
    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");

    parser::untyped_example(&contents);
}
