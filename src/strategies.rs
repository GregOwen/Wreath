pub const DEFAULT_STRATEGY: &str = "FIRST_N";
pub const REPLACEMENT_STRATEGY_ENV_VAR: &str = "GITFUN_STRATEGY";

/// Select the strategy that we will use to rename the commits. The strategy will be a function that,
/// given the total blob that we have as our replacement text and the number of commits for which we
/// need replacement messages, returns a list of the replacement messages we should use for the
/// commits.
pub fn get_replacement_lines_strategy<'a>(
        strategy_name: &'a str) -> fn(&'a str, usize) -> Vec<&'a str> {
    match strategy_name {
        "FIRST_N" => get_replacement_lines,
        _ => panic!("Could not find strategy with the name '{}'", strategy_name),
    }
}

/// Simple strategy: just take the first num_commits lines from replacement_contents.
fn get_replacement_lines<'a>(replacement_contents: &'a str, num_commits: usize) -> Vec<&'a str> {
    // TODO(greg): handle the case where we have more commits than replacement lines
    replacement_contents.split("\n").take(num_commits).collect()
}
