extern crate env_logger;
extern crate gitfun;
#[macro_use] extern crate log;

use std::env;

fn main() {
    // Start logging
    env_logger::init().unwrap();

    // Parse args
    let args: Vec<String> = env::args().collect();
    debug!("sequence_editor args: {:?}", args);
    let config = parse_config(&args);

    // Read in replacement file
    let replacement_contents = gitfun::read_file_contents(&config.replacement_filepath);

    debug!("Got replacement contents:\n{}", replacement_contents);
    debug!("");

    // Read in rebase file
    let rebase_contents = gitfun::read_file_contents(&config.rebase_filepath);
    debug!("Got rebase contents:\n{}", rebase_contents);

    // Write out new commit message file
    let new_commit_messages = get_new_commit_messages(
        &replacement_contents, &rebase_contents, &config.replacement_strategy);
    gitfun::write_str_to_file(&new_commit_messages, gitfun::TRACKER_FILE_NAME);
    debug!("Got new commit messages:\n{}", new_commit_messages);

    // Write out new rebase file
    let new_rebase_contents = get_new_rebase_contents(&rebase_contents);
    gitfun::write_str_to_file(&new_rebase_contents, &config.rebase_filepath);
}

struct Config {
    rebase_filepath: String,
    replacement_filepath: String,
    replacement_strategy: String,
}

fn parse_config(args: &[String]) -> Config {
    let replacement_filepath = args[1].clone();
    let rebase_filepath = args[2].clone();

    let replacement_strategy = env::var(gitfun::strategies::REPLACEMENT_STRATEGY_ENV_VAR)
        .unwrap_or(String::from(gitfun::strategies::DEFAULT_STRATEGY));

    Config { rebase_filepath, replacement_filepath, replacement_strategy }
}

fn get_new_commit_messages(
        replacement_contents: &str, rebase_contents: &str, replacement_strategy: &str) -> String {
    let commit_line_blob = rebase_contents.split("\n\n").next()
        .expect(&format!("Rebase contents '{}' contains no commit lines", rebase_contents));

    let num_commit_lines = commit_line_blob.split("\n").count();
    let get_replacement_lines =
        gitfun::strategies::get_replacement_lines_strategy(replacement_strategy);
    let replacement_lines = get_replacement_lines(replacement_contents, num_commit_lines);
    prepare_replacements_for_output(replacement_lines)
}

fn prepare_replacements_for_output(replacement_lines: Vec<&str>) -> String {
    // Explicit type annotation necessary
    let output_lines: Vec<String> = replacement_lines.iter()
        .rev() // Write out commits in reverse order so that earliest commit gets last line of msg
        .map(|line| [gitfun::TODO_PREFIX, line].concat())
        .collect();
    output_lines.join("\n")
}

fn get_new_rebase_contents(old_rebase_contents: &str) -> String {
    old_rebase_contents.clone().replace("pick", "reword")
}
