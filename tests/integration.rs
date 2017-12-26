extern crate env_logger;
extern crate gitfun;
#[macro_use] extern crate log;
extern crate tempdir;

use std::path::Path;

mod gitfun_test_utils;

const NEW_COMMIT_MESSAGE_CONTENTS: &str = "There once was a man from the West
Who wrote the most shallow of tests
When his colleagues Reacted
He quickly retracted
Said he, 'It was written in Jest!'";
const NUM_COMMITS: usize = 5;

#[test]
fn test_default_is_first_n() {
    let config = gitfun_test_utils::TestConfig {
        new_commit_message_contents: NEW_COMMIT_MESSAGE_CONTENTS,
        num_commits: NUM_COMMITS,
        strategy: None,
        verify_new_messages: verify_new_messages_first_n,
    };
    gitfun_test_utils::end_to_end_test(config);
}

#[test]
fn test_first_n() {
    let config = gitfun_test_utils::TestConfig {
        new_commit_message_contents: NEW_COMMIT_MESSAGE_CONTENTS,
        num_commits: NUM_COMMITS,
        strategy: Some("FIRST_N"),
        verify_new_messages: verify_new_messages_first_n,
    };
    gitfun_test_utils::end_to_end_test(config);
}

fn verify_new_messages_first_n(dir_path: &Path, new_commit_message_contents: &str) {
    let dir_str = dir_path.to_str();

    // Correct git commit messages
    let git_res = gitfun::exec_command("git log --pretty=format:%s", dir_str).stdout;
    let commits = String::from_utf8_lossy(&git_res);
    assert_eq!(*commits.trim_right(), *new_commit_message_contents);
}
