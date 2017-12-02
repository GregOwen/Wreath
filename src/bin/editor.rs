extern crate gitfun;

use std::env;
use std::process::Command;

fn main() {
    // Parse args
    let args: Vec<String> = env::args().collect();
    println!("editor args: {:?}", args);
    let filepath = args[1].clone();

    // Get message we need to write
    let msg = get_commit_message();
    println!("Got message:\n{}", msg);

    // Write message to file
    gitfun::write_str_to_file(&msg, &filepath);
}

fn get_commit_message() -> String {
    let env_path = env::var("PATH")
        .expect(&format!("Input environment {:?} had no PATH var", env::vars()));

    // We use the equivalent of EDITOR=cat git rebase --edit-todo | head -1
    // This should look like "reword $HASH $MSG"
    let cmd = Command::new("git")
        .env_clear()
        .env("PATH", env_path)
        .env("EDITOR", "cat")
        .arg("rebase")
        .arg("--edit-todo")
        .output()
        .expect("Failed to execute git command");

    let stdout_str = String::from_utf8_lossy(&cmd.stdout);
    let commit_line = stdout_str.split("\n").next()
        .expect(&format!("stdout_str was empty!"));
    // Explicit type annotation necessary
    let commit_words: Vec<&str> = commit_line.split(" ").skip(2).collect();

    commit_words.join(" ")
}
