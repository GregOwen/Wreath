extern crate gitfun;
extern crate tempdir;

use std::path::Path;
use tempdir::TempDir;

const NEW_COMMIT_MESSAGE_CONTENTS: &str = "There once was a man from the West
Who wrote the most shallow of tests
When his colleagues Reacted
He quickly retracted
Said he, 'It was written in Jest!'";
const NEW_COMMIT_MESSAGE_FILE: &str = "new_message.txt";
const NUM_COMMITS: usize = 5;

#[test]
fn end_to_end_test() {
    // create temp dir
    let temp_dir = match TempDir::new("end_to_end_test") {
        Ok(dir) => dir,
        e => panic!("{:?}", e), // Just fail fast if we can't create the temp dir
    };
    let temp_dir_path = temp_dir.path();
    println!("temp_dir is {:?}", temp_dir_path.to_str());

    // set up git history in temp dir
    setup_temp_dir(&temp_dir_path);

    // verify setup worked
    verify_setup(&temp_dir_path);

    // run binary
    run_binary(&temp_dir_path);

    // verify binary had desired effect
    verify_new_messages(&temp_dir_path);
}

fn setup_temp_dir(dir_path: &Path) {
    gitfun::exec_command("git init", dir_path.to_str());
    setup_message_file(&dir_path);
    setup_commit_files(&dir_path);
}

fn setup_message_file(dir_path: &Path) {
    let file_path = &dir_path.join(NEW_COMMIT_MESSAGE_FILE);
    let file_name = file_path.to_string_lossy();
    gitfun::write_str_to_file(NEW_COMMIT_MESSAGE_CONTENTS, &file_name);
    gitfun::exec_command(
        &format!("git add {} && git commit -m 'message file'", file_name),
        dir_path.to_str());
}

fn setup_commit_files(dir_path: &Path) {
    for i in 1..NUM_COMMITS {
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

fn verify_setup(dir_path: &Path) {
    let dir_str = dir_path.to_str();
    let ls_res = gitfun::exec_command("ls", dir_str).stdout;
    let files = String::from_utf8_lossy(&ls_res);
    assert_eq!(files.trim_right().split("\n").collect::<Vec<&str>>().len(), NUM_COMMITS);

    let git_res = gitfun::exec_command("git log --pretty=oneline", dir_str).stdout;
    let commits = String::from_utf8_lossy(&git_res);
    assert_eq!(commits.trim_right().split("\n").collect::<Vec<&str>>().len(), NUM_COMMITS);
}

fn run_binary(dir_path: &Path) {
    let dir_str = dir_path.to_str();
    let executable_path = get_executable_path();
    println!("Got executable_path: {:?}", executable_path);
    let command_str = format!("{} {}", executable_path, NEW_COMMIT_MESSAGE_FILE);
    gitfun::exec_command(&command_str, dir_str);
}

fn get_executable_path() -> String {
    let root_dir_res = gitfun::exec_command("git rev-parse --show-toplevel", None).stdout;
    let root_dir = String::from_utf8_lossy(&root_dir_res);
    let root_dir_path = Path::new(root_dir.trim_right());
    root_dir_path.join("target").join("debug").join("gitfun").to_string_lossy().into_owned()
}

fn verify_new_messages(dir_path: &Path) {
    let dir_str = dir_path.to_str();

    // Correct git commit messages
    let git_res = gitfun::exec_command("git log --pretty=format:%s", dir_str).stdout;
    let commits = String::from_utf8_lossy(&git_res);
    assert_eq!(commits.trim_right(), NEW_COMMIT_MESSAGE_CONTENTS);

    // No new files
    let ls_res = gitfun::exec_command("ls -a", dir_str).stdout;
    let files = String::from_utf8_lossy(&ls_res);

    // . .. .git
    let desired_num_files = NUM_COMMITS + 3;
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