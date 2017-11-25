use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let filepath = &args[1];

    println!("Received file '{}'", filepath);
    let mut f = File::open(filepath)
        .expect(&format!("Could not open file at path '{}'", filepath));

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect(&format!("Error reading file at path '{}'", filepath));

    println!("Got contents:\n{}", contents);
}
