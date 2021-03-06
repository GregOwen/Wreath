extern crate env_logger;
extern crate wreath;
#[macro_use] extern crate log;

use std::env;

fn main() {
    // Start logging
    env_logger::init().unwrap();

    // Parse args
    let args: Vec<String> = env::args().collect();
    debug!("editor args: {:?}", args);
    let filepath = args[1].clone();

    // Get message we need to write
    let msg = next_commit_message();
    debug!("Got message:\n{}", msg);

    // Write message to file
    wreath::write_str_to_file(&msg, &filepath);
}

/// This will read the next commit message from the tracker file and mark it as read.
/// DO NOT call this function more than once.
fn next_commit_message() -> String {
    let tracker_contents = wreath::read_file_contents(wreath::TRACKER_FILE_NAME);
    let next_message = get_next_message(&tracker_contents);

    let new_tracker_contents = consume_one_message(&tracker_contents);
    wreath::write_str_to_file(&new_tracker_contents, wreath::TRACKER_FILE_NAME);

    next_message
}

fn get_next_message(tracker_contents: &str) -> String {
    // Explicit type annotation necessary
    let raw_message_lines: Vec<&str> = tracker_contents.lines().collect();
    let message_line = raw_message_lines.iter()
        .find(|&&line| line.starts_with(wreath::TODO_PREFIX))
        .expect(&format!(
            "Could not get next message: found no message lines that began with '{}'",
            wreath::TODO_PREFIX));
    message_line.split_at(wreath::TODO_PREFIX.len()).1.to_string()
}

fn consume_one_message(old_tracker_contents: &str) -> String {
    if !old_tracker_contents.contains(wreath::TODO_PREFIX) {
        panic!(
            "Could not consume message: found no message lines that began with '{}'",
            wreath::TODO_PREFIX);
    }
    old_tracker_contents.clone().replacen(wreath::TODO_PREFIX, wreath::DONE_PREFIX, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_next_message_works_when_all_lines_todo() {
        let input = format!(
            "{}Test line 1\n{}Test line 2\n{}Test line 3",
            wreath::TODO_PREFIX, wreath::TODO_PREFIX, wreath::TODO_PREFIX);
        let next_message = get_next_message(&input);
        assert_eq!(next_message, "Test line 1");
    }

    #[test]
    fn get_next_message_works_when_some_line_done() {
        let input = format!(
            "{}Test line 1\n{}Test line 2\n{}Test line 3",
            wreath::DONE_PREFIX, wreath::TODO_PREFIX, wreath::TODO_PREFIX);
        let next_message = get_next_message(&input);
        assert_eq!(next_message, "Test line 2");
    }

    #[test]
    #[should_panic]
    fn get_next_message_panics_when_all_lines_done() {
        let input = format!(
            "{}Test line 1\n{}Test line 2\n{}Test line 3",
            wreath::DONE_PREFIX, wreath::DONE_PREFIX, wreath::DONE_PREFIX);
        get_next_message(&input);
    }

    #[test]
    fn consume_one_message_works_when_all_lines_todo() {
        let input = format!(
            "{}Test line 1\n{}Test line 2\n{}Test line 3",
            wreath::TODO_PREFIX, wreath::TODO_PREFIX, wreath::TODO_PREFIX);
        let expected = format!(
            "{}Test line 1\n{}Test line 2\n{}Test line 3",
            wreath::DONE_PREFIX, wreath::TODO_PREFIX, wreath::TODO_PREFIX);
        let after_consumption = consume_one_message(&input);
        assert_eq!(expected, after_consumption);
    }

    #[test]
    fn consume_one_message_works_when_some_line_done() {
        let input = format!(
            "{}Test line 1\n{}Test line 2\n{}Test line 3",
            wreath::DONE_PREFIX, wreath::TODO_PREFIX, wreath::TODO_PREFIX);
        let expected = format!(
            "{}Test line 1\n{}Test line 2\n{}Test line 3",
            wreath::DONE_PREFIX, wreath::DONE_PREFIX, wreath::TODO_PREFIX);
        let after_consumption = consume_one_message(&input);
        assert_eq!(expected, after_consumption);
    }

    #[test]
    #[should_panic]
    fn consume_one_message_panics_when_all_lines_done() {
        let input = format!(
            "{}Test line 1\n{}Test line 2\n{}Test line 3",
            wreath::DONE_PREFIX, wreath::DONE_PREFIX, wreath::DONE_PREFIX);
        consume_one_message(&input);
    }
}
