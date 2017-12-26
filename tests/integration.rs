extern crate env_logger;
extern crate gitfun;
#[macro_use] extern crate log;
extern crate tempdir;

mod gitfun_test_utils;

const NEW_COMMIT_MESSAGE_CONTENTS: &str = "There once was a man from the West
Who wrote the most shallow of tests
When his colleagues Reacted
He quickly retracted
Said he, 'It was written in Jest!'";

#[test]
fn test_default_is_first_n() {
    let expected = format!("{}\ncommit 1\ncommit 0", NEW_COMMIT_MESSAGE_CONTENTS);
    let config = gitfun_test_utils::TestConfig {
        new_commit_message_contents: NEW_COMMIT_MESSAGE_CONTENTS,
        num_commits: 7,
        strategy: None,
        expected_new_messages: &expected,
    };
    gitfun_test_utils::end_to_end_test(config);
}

#[test]
fn test_first_n_more_commits() {
    let expected = format!("{}\ncommit 1\ncommit 0", NEW_COMMIT_MESSAGE_CONTENTS);
    let config = gitfun_test_utils::TestConfig {
        new_commit_message_contents: NEW_COMMIT_MESSAGE_CONTENTS,
        num_commits: 7,
        strategy: Some("FIRST_N"),
        expected_new_messages: &expected,
    };
    gitfun_test_utils::end_to_end_test(config);
}

#[test]
fn test_first_n_more_messages() {
    let first_3_lines: Vec<&str> = NEW_COMMIT_MESSAGE_CONTENTS.lines().take(3).collect();
    let first_3 = String::from(first_3_lines.join("\n"));
    let config = gitfun_test_utils::TestConfig {
        new_commit_message_contents: NEW_COMMIT_MESSAGE_CONTENTS,
        num_commits: 3,
        strategy: Some("FIRST_N"),
        expected_new_messages: &first_3,
    };
    gitfun_test_utils::end_to_end_test(config);
}

