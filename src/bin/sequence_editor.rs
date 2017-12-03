extern crate gitfun;

use std::env;

fn main() {
    // Parse args
    let args: Vec<String> = env::args().collect();
    println!("sequence_editor args: {:?}", args);
    let config = parse_config(&args);

    // Read in replacement file
    let replacement_contents = gitfun::read_file_contents(&config.replacement_filepath);

    println!("Got replacement contents:\n{}", replacement_contents);
    println!("");

    // Read in rebase file
    let rebase_contents = gitfun::read_file_contents(&config.rebase_filepath);
    println!("Got rebase contents:\n{}", rebase_contents);

    // Write out new commit message file
    let new_commit_messages = get_new_commit_messages(&replacement_contents, &rebase_contents);
    // TODO(greg): put this in a tmp file and clean it up
    gitfun::write_str_to_file(&new_commit_messages, gitfun::TRACKER_FILE_NAME);
    println!("Got new commit messages:\n{}", new_commit_messages);

    // Write out new rebase file
    let new_rebase_contents = get_new_rebase_contents(&rebase_contents);
    gitfun::write_str_to_file(&new_rebase_contents, &config.rebase_filepath);
}

struct Config {
    replacement_filepath: String,
    rebase_filepath: String,
}

fn parse_config(args: &[String]) -> Config {
    let replacement_filepath = args[1].clone();
    let rebase_filepath = args[2].clone();

    Config { replacement_filepath, rebase_filepath }
}

fn get_new_commit_messages(replacement_contents: &str, rebase_contents: &str) -> String {
    let commit_line_blob = rebase_contents.split("\n\n").next()
        .expect(&format!("Rebase contents '{}' contains no commit lines", rebase_contents));

    let num_commit_lines = commit_line_blob.split("\n").count();
    let replacement_lines = get_replacement_lines(replacement_contents, num_commit_lines);
    prepare_replacements_for_output(replacement_lines)
}

/// Given the total blob that we have as our replacement text and the number of commits for which we
/// need replacement messages, return a list of the replacement messages we should use for the
/// commits. Eventually, this will allow multiple strategies. For now, we just take the first
/// num_commits lines from replacement_contents.
fn get_replacement_lines(replacement_contents: &str, num_commits: usize) -> Vec<&str> {
    // TODO(greg): handle the case where we have more commits than replacement lines
    replacement_contents.split("\n").take(num_commits).collect()
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
