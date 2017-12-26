pub const DEFAULT_STRATEGY: &str = "FIRST_N";
pub const REPLACEMENT_STRATEGY_ENV_VAR: &str = "GITFUN_STRATEGY";

/// The strategy we will use to determine what the new commit messages should be.
/// # Arguments
///
/// * `replacement_contents` - The contents of the replacement text file as a single string slice
/// * `num_commits` - The number of commits for which we need replacement messages
///
/// # Returns
///
/// A vector containing the new commit messages. Strategies may return a vector of any length <=
/// num_commits. If a strategy returns strictly fewer messages than we have commits, the most
/// recent commits will have their messages overwritten and the remaining commits will have their
/// messages unchanged.
pub type Strategy = fn(&str, usize) -> Vec<&str>;

/// Select the strategy that we will use to rename the commits
pub fn get_replacement_lines_strategy(strategy_name: &str) -> Strategy {
    match strategy_name {
        "FIRST_N" => first_n,
        _ => panic!("Could not find strategy with the name '{}'", strategy_name),
    }
}

/// Simple strategy: just take the first num_commits lines from replacement_contents.
fn first_n<'a>(replacement_contents: &'a str, num_commits: usize) -> Vec<&'a str> {
    replacement_contents.lines().take(num_commits).collect()
}
