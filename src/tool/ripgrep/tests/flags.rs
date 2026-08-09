//! Flag-building tests: [`RgArgs`] to ripgrep argument vector.

use super::helpers::args;
use crate::tool::ripgrep::command;

#[test]
fn pattern_is_placed_after_double_dash() {
    let built = command::build(&args("-weird"));
    let separator = built.iter().position(|a| a == "--").expect("has --");
    assert_eq!(built[separator + 1], "-weird");
}

#[test]
fn defaults_search_current_directory() {
    let built = command::build(&args("x"));
    assert_eq!(built.last().map(String::as_str), Some("."));
}

#[test]
fn glob_filters_become_repeated_flags() {
    let mut input = args("x");
    input.glob = vec!["*.rs".into(), "!target/**".into()];
    let built = command::build(&input);
    assert!(built.contains(&"--glob=*.rs".to_string()));
    assert!(built.contains(&"--glob=!target/**".to_string()));
}

#[test]
fn explicit_paths_replace_the_default() {
    let mut input = args("x");
    input.paths = vec!["src".into(), "docs".into()];
    let built = command::build(&input);
    assert!(built.contains(&"src".to_string()));
    assert!(!built.contains(&".".to_string()));
}

#[test]
fn limits_are_clamped_to_ceilings() {
    let mut input = args("x");
    input.limit = Some(usize::MAX);
    assert_eq!(input.limit(), crate::tool::ripgrep::args::MAX_LIMIT);
    input.timeout_secs = Some(u64::MAX);
    assert_eq!(
        input.timeout().as_secs(),
        crate::tool::ripgrep::args::MAX_TIMEOUT_SECS
    );
}
