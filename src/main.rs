use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // Parse args
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let config = parse_config(&args);

    // Read in replacement file
    let replacement_contents = read_file_contents(&config.replacement_filepath);

    println!("Got replacement contents:\n{}", replacement_contents);
    println!("");

    // Read in rebase file
    let rebase_contents = read_file_contents(&config.rebase_filepath);
    println!("Got rebase contents:\n{}", rebase_contents);

    // Write out new rebase file
    let rewritten_contents = get_rewritten_contents(&replacement_contents, &rebase_contents);
    println!("Got rewritten contents:\n{}", rewritten_contents);

    //   Format replacement file
    //   Write formatted replacement

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

fn read_file_contents(filepath: &str) -> String {
    println!("Received file '{}'", filepath);
    let mut f = File::open(filepath)
        .expect(&format!("Could not open file at path '{}'", filepath));

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect(&format!("Error reading file at path '{}'", filepath));

    contents
}

fn get_rewritten_contents(replacement_contents: &str, rebase_contents: &str) -> String {
    let commit_line_blob = rebase_contents.split("\n\n").next()
        .expect(&format!("Rebase contents '{}' contains no commit lines", rebase_contents));

    // Explicit type annotation necessary
    let commit_lines: Vec<&str> = commit_line_blob.split("\n").collect();
    let replacement_lines = get_replacement_lines(replacement_contents, commit_lines.len());
    rewrite_commit_lines(commit_lines, replacement_lines)
}

/// Given the total blob that we have as our replacement text and the number of commits for which we
/// need replacement messages, return a list of the replacement messages we should use for the
/// commits. Eventually, this will allow multiple strategies. For now, we just take the first
/// num_commits lines from replacement_contents.
fn get_replacement_lines(replacement_contents: &str, num_commits: usize) -> Vec<&str> {
    replacement_contents.split("\n").take(num_commits).collect()
}

fn rewrite_commit_lines(commit_lines: Vec<&str>, replacement_lines: Vec<&str>) -> String {
    let mut output: String = String::new();
    for (i, commit_line) in commit_lines.iter().enumerate() {
        // If we have more commits than replacement lines, just loop
        let replacement_idx = i % (replacement_lines.len());
        let replacement_line = replacement_lines[replacement_idx];
        // Commit lines look like "pick $HASH $MESSAGE"
        let commit_hash = commit_line.split_whitespace().nth(1)
            .expect(&format!("Failed to parse hash code from line '{}'", commit_line));

        output.push_str(&format!("reword {} {}\n", commit_hash, replacement_line))
    };
    output
}
