#[macro_use] extern crate log;

use std::fs::File;
use std::io::prelude::*;
use std::process::{Command, Output, Stdio};

pub const DONE_PREFIX: &str = "DONE||";
pub const TODO_PREFIX: &str = "TODO||";

pub const EDITOR_BINARY_NAME: &str = "editor";
pub const SEQUENCE_EDITOR_BINARY_NAME: &str = "sequence_editor";
pub const TRACKER_FILE_NAME: &str = ".wreath_tracker";

pub fn exec_command(command_string: &str, dir: Option<&str>) -> Output {
    let mut base_cmd: Command = Command::new("bash");
    let cmd: &mut Command = base_cmd
        .arg("-c")
        .arg(command_string)
        .current_dir(dir.unwrap_or("."))
        .stderr(Stdio::inherit());

    trace!("running command: {:?}", cmd);

    cmd
        .output()
        .expect("command failed to start")
}

pub fn read_file_contents(filepath: &str) -> String {
    trace!("Received file '{}'", filepath);
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

pub mod strategies;
