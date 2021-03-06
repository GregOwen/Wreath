extern crate env_logger;
extern crate wreath;
#[macro_use] extern crate log;

use std::env;
use std::path::Path;

fn main() {
    // Start logging
    env_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();
    debug!("wreath args: {:?}", args);
    let config = parse_config(&args);

    invoke_git_command(config);
    // TODO(greg): better exception handling + do this as a finally
    clean();
}

struct Config {
    editor_binpath: String,
    sequence_editor_binpath: String,
    replacement_filepath: String,
}

fn parse_config(args: &[String]) -> Config {
    // Get path to other binaries. Assume they are in the same directory as this binary.
    let this_binpath = Path::new(&args[0]);
    let editor_binpath = this_binpath.with_file_name(wreath::EDITOR_BINARY_NAME)
        .to_string_lossy().into_owned();
    let sequence_editor_binpath = this_binpath.with_file_name(wreath::SEQUENCE_EDITOR_BINARY_NAME)
        .to_string_lossy().into_owned();

    // Get path to replacement messages
    let replacement_filepath = args[1].clone();

    Config { editor_binpath, sequence_editor_binpath, replacement_filepath }
}

fn invoke_git_command(config: Config) {
    let command_string = format!(
        "EDITOR='{}' GIT_SEQUENCE_EDITOR='{} {}' git rebase -i --root",
        &config.editor_binpath,
        &config.sequence_editor_binpath,
        &config.replacement_filepath);
    debug!("Executing top-level command '{}'", &command_string);
    wreath::exec_command(&command_string, None);
}

fn clean() {
    // Remove the tracker file
    wreath::exec_command(&format!("rm {}", wreath::TRACKER_FILE_NAME), None);
}
