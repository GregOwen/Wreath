pub fn get_replacement_lines_strategy<'a>(
        strategy_name: &'a str) -> fn(&'a str, usize) -> Vec<&'a str> {
    match strategy_name {
        "FIRST_N" => get_replacement_lines,
        _ => panic!("Could not find strategy with the name '{}'", strategy_name),
    }
}

/// Given the total blob that we have as our replacement text and the number of commits for which we
/// need replacement messages, return a list of the replacement messages we should use for the
/// commits. Eventually, this will allow multiple strategies. For now, we just take the first
/// num_commits lines from replacement_contents.
fn get_replacement_lines<'a>(replacement_contents: &'a str, num_commits: usize) -> Vec<&'a str> {
    // TODO(greg): handle the case where we have more commits than replacement lines
    replacement_contents.split("\n").take(num_commits).collect()
}
