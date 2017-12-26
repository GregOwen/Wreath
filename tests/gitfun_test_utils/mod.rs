extern crate gitfun;
extern crate tempdir;

use std::path::Path;
use tempdir::TempDir;

const NEW_COMMIT_MESSAGE_FILE: &str = "new_message.txt";

pub struct TestConfig<'a> {
    pub new_commit_message_contents: &'a str,
    pub num_commits: usize,
    pub strategy: &'a str,
    pub verify_new_messages: fn(&Path, &str),
}

pub fn end_to_end_test(config: TestConfig) {
    // create temp dir
    let temp_dir = match TempDir::new("end_to_end_test") {
        Ok(dir) => dir,
        e => panic!("{:?}", e), // Just fail fast if we can't create the temp dir
    };
    let temp_dir_path = temp_dir.path();
    debug!("temp_dir is {:?}", temp_dir_path.to_str());

    // set up git history in temp dir
    setup_temp_dir(&temp_dir_path, config.new_commit_message_contents, config.num_commits);

    // verify setup worked
    verify_setup(&temp_dir_path, config.num_commits);

    // run binary
    run_binary(&temp_dir_path, config.strategy);

    // verify binary had desired effect
    (config.verify_new_messages)(&temp_dir_path, config.new_commit_message_contents);
    verify_no_changes(&temp_dir_path, config.num_commits);
}

fn setup_temp_dir(dir_path: &Path, new_commit_message_contents: &str, num_commits: usize) {
    gitfun::exec_command("git init", dir_path.to_str());
    setup_message_file(&dir_path, new_commit_message_contents);
    setup_commit_files(&dir_path, num_commits);
}

fn setup_message_file(dir_path: &Path, new_commit_message_contents: &str) {
    let file_path = &dir_path.join(NEW_COMMIT_MESSAGE_FILE);
    let file_name = file_path.to_string_lossy();
    gitfun::write_str_to_file(new_commit_message_contents, &file_name);
    gitfun::exec_command(
        &format!("git add {} && git commit -m 'message file'", file_name),
        dir_path.to_str());
}

fn setup_commit_files(dir_path: &Path, num_commits: usize) {
    for i in 1..num_commits {
        add_file_and_commit(dir_path, i);
    }
}

fn add_file_and_commit(dir_path: &Path, commit_num: usize) {
    let file_name = format!("test{}.txt", commit_num);
    gitfun::write_str_to_file(
        &format!("Test file {} contents", commit_num),
        &dir_path.join(&file_name).to_string_lossy());
    gitfun::exec_command(
        &format!("git add {} && git commit -m 'commit {}'", file_name, commit_num),
        dir_path.to_str());
}

fn verify_setup(dir_path: &Path, num_commits: usize) {
    let dir_str = dir_path.to_str();
    let ls_res = gitfun::exec_command("ls", dir_str).stdout;
    let files = String::from_utf8_lossy(&ls_res);
    trace!("verify_setup created files:\n{}", files);
    assert_eq!(files.trim_right().split("\n").collect::<Vec<&str>>().len(), num_commits);

    let git_res = gitfun::exec_command("git log --pretty=oneline", dir_str).stdout;
    let commits = String::from_utf8_lossy(&git_res);
    trace!("verify_setup created commits:\n{}", commits);
    assert_eq!(commits.trim_right().split("\n").collect::<Vec<&str>>().len(), num_commits);
}

fn run_binary(dir_path: &Path, strategy: &str) {
    let dir_str = dir_path.to_str();
    let executable_path = get_executable_path();
    debug!("Got executable_path: {:?}", executable_path);
    let command_str = format!(
        "{}='{}' {} {}",
        gitfun::strategies::REPLACEMENT_STRATEGY_ENV_VAR,
        strategy,
        executable_path,
        NEW_COMMIT_MESSAGE_FILE);
    gitfun::exec_command(&command_str, dir_str);
}

fn get_executable_path() -> String {
    let root_dir_res = gitfun::exec_command("git rev-parse --show-toplevel", None).stdout;
    let root_dir = String::from_utf8_lossy(&root_dir_res);
    let root_dir_path = Path::new(root_dir.trim_right());
    root_dir_path.join("target").join("debug").join("gitfun").to_string_lossy().into_owned()
}

fn verify_no_changes(dir_path: &Path, num_commits: usize) {
    let dir_str = dir_path.to_str();

    // No new files
    let ls_res = gitfun::exec_command("ls -a", dir_str).stdout;
    let files = String::from_utf8_lossy(&ls_res);

    // . .. .git
    let desired_num_files = num_commits + 3;
    let observed_num_files = files.trim_right().split("\n").collect::<Vec<&str>>().len();
    assert_eq!(
        observed_num_files,
        desired_num_files,
        "Expected {} files but found {}.
$ ls -a
{}",
        desired_num_files,
        observed_num_files,
        files
    );

    // No changes to existing files
    let status_res = gitfun::exec_command("git status -s", dir_str).stdout;
    let status = String::from_utf8_lossy(&status_res);
    assert_eq!(status.trim_right(), "");
}
