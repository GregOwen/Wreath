/// Given the total blob that we have as our replacement text and the number of commits for which we
/// need replacement messages, return a list of the replacement messages we should use for the
/// commits. Eventually, this will allow multiple strategies. For now, we just take the first
/// num_commits lines from replacement_contents.
pub fn get_replacement_lines(replacement_contents: &str, num_commits: usize) -> Vec<&str> {
    // TODO(greg): handle the case where we have more commits than replacement lines
    replacement_contents.split("\n").take(num_commits).collect()
}
