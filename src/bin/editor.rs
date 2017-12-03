extern crate gitfun;

use std::env;

fn main() {
    // Parse args
    let args: Vec<String> = env::args().collect();
    println!("editor args: {:?}", args);
    let filepath = args[1].clone();

    // Get message we need to write
    let msg = next_commit_message();
    println!("Got message:\n{}", msg);

    // Write message to file
    gitfun::write_str_to_file(&msg, &filepath);
}

/// This will read the next commit message from the tracker file and mark it as read.
/// DO NOT call this function more than once.
fn next_commit_message() -> String {
    let tracker_contents = gitfun::read_file_contents(gitfun::TRACKER_FILE_NAME);
    let next_message = get_next_message(&tracker_contents);

    let new_tracker_contents = consume_one_message(&tracker_contents);
    gitfun::write_str_to_file(&new_tracker_contents, gitfun::TRACKER_FILE_NAME);

    next_message
}

fn get_next_message(tracker_contents: &str) -> String {
    // Explicit type annotation necessary
    let raw_message_lines: Vec<&str> = tracker_contents.split("\n").collect();
    let message_line = raw_message_lines.iter()
        .find(|&&line| line.starts_with(gitfun::TODO_PREFIX))
        .expect(&format!("Found no message lines that began with '{}'", gitfun::TODO_PREFIX));
    message_line.split_at(gitfun::TODO_PREFIX.len()).1.to_string()
}

fn consume_one_message(old_tracker_contents: &str) -> String {
    old_tracker_contents.clone().replacen(gitfun::TODO_PREFIX, gitfun::DONE_PREFIX, 1)
}
