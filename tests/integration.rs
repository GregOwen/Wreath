extern crate env_logger;
extern crate wreath;
#[macro_use] extern crate log;
extern crate tempdir;

mod wreath_test_utils;

const NEW_COMMIT_MESSAGE_CONTENTS: &str = "There once was a man from the West
Who wrote the most shallow of tests
When his colleagues Reacted
He quickly retracted
Said he, 'It was written in Jest!'";

#[test]
fn test_cycle_more_commits() {
    let expected = format!(
        "{}\n{}\n{}\n{}",
        NEW_COMMIT_MESSAGE_CONTENTS,
        NEW_COMMIT_MESSAGE_CONTENTS,
        "There once was a man from the West",
        "Who wrote the most shallow of tests",
    );
    let config = wreath_test_utils::TestConfig {
        new_commit_message_contents: NEW_COMMIT_MESSAGE_CONTENTS,
        num_commits: 12,
        strategy: Some("CYCLE"),
        expected_new_messages: &expected,
    };
    wreath_test_utils::end_to_end_test(config);
}

#[test]
fn test_cycle_more_messages() {
    let expected = "There once was a man from the West
Who wrote the most shallow of tests
When his colleagues Reacted";
    let config = wreath_test_utils::TestConfig {
        new_commit_message_contents: NEW_COMMIT_MESSAGE_CONTENTS,
        num_commits: 3,
        strategy: Some("CYCLE"),
        expected_new_messages: expected,
    };
    wreath_test_utils::end_to_end_test(config);
}

#[test]
fn test_default_is_first_n() {
    let expected = format!("{}\ncommit 1\ncommit 0", NEW_COMMIT_MESSAGE_CONTENTS);
    let config = wreath_test_utils::TestConfig {
        new_commit_message_contents: NEW_COMMIT_MESSAGE_CONTENTS,
        num_commits: 7,
        strategy: None,
        expected_new_messages: &expected,
    };
    wreath_test_utils::end_to_end_test(config);
}

#[test]
fn test_first_n_more_commits() {
    let expected = format!("{}\ncommit 1\ncommit 0", NEW_COMMIT_MESSAGE_CONTENTS);
    let config = wreath_test_utils::TestConfig {
        new_commit_message_contents: NEW_COMMIT_MESSAGE_CONTENTS,
        num_commits: 7,
        strategy: Some("FIRST_N"),
        expected_new_messages: &expected,
    };
    wreath_test_utils::end_to_end_test(config);
}

#[test]
fn test_first_n_more_messages() {
    let expected = "There once was a man from the West
Who wrote the most shallow of tests
When his colleagues Reacted";
    let config = wreath_test_utils::TestConfig {
        new_commit_message_contents: NEW_COMMIT_MESSAGE_CONTENTS,
        num_commits: 3,
        strategy: Some("FIRST_N"),
        expected_new_messages: expected,
    };
    wreath_test_utils::end_to_end_test(config);
}

