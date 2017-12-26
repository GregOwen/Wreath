pub const DEFAULT_STRATEGY: &str = "FIRST_N";
pub const REPLACEMENT_STRATEGY_ENV_VAR: &str = "GITFUN_STRATEGY";

// TODO(greg): pull out strategy as a type alias
/// Select the strategy that we will use to rename the commits. The strategy is a function that,
/// given the total blob that we have as our replacement text and the number of commits for which we
/// need replacement messages, returns a list of the replacement messages we should use for the
/// commits. Strategies may return a vector of any length <= the number of commits. If a strategy
/// returns a vector of replacement messages with strictly fewer messages than we have commits, the
/// most recent commits will have their messages overwritten and the remaining commits will have
/// their messages unchanged.
pub fn get_replacement_lines_strategy<'a>(
        strategy_name: &'a str) -> fn(&'a str, usize) -> Vec<&'a str> {
    match strategy_name {
        "FIRST_N" => get_replacement_lines,
        _ => panic!("Could not find strategy with the name '{}'", strategy_name),
    }
}

/// Simple strategy: just take the first num_commits lines from replacement_contents.
fn get_replacement_lines<'a>(replacement_contents: &'a str, num_commits: usize) -> Vec<&'a str> {
    replacement_contents.split("\n").take(num_commits).collect()
}
