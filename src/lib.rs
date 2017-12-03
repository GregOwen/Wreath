use std::fs::File;
use std::io::prelude::*;

pub const DONE_PREFIX: &str = "DONE||";
pub const TODO_PREFIX: &str = "TODO||";

pub const TRACKER_FILE_NAME: &str = ".gitfun_tracker";

pub fn read_file_contents(filepath: &str) -> String {
    println!("Received file '{}'", filepath);
    let mut f = File::open(filepath)
        .expect(&format!("Could not open file at path '{}'", filepath));

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect(&format!("Error reading file at path '{}'", filepath));

    contents
}

pub fn write_str_to_file(contents: &str, filepath: &str) {
    let mut f = File::create(filepath)
        .expect(&format!("Could not create file at path '{}'", filepath));

    f.write_all(contents.as_bytes())
        .expect(&format!("Could not write to file at path '{}'", filepath));
}
