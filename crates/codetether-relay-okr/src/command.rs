//! Recognize relay easy-command names.

/// Check whether the input is a `/go` or `/team` easy command.
/// # Arguments
/// * `input` - Full user command text.
/// # Returns
/// True when the first token names go or team, ignoring case.
/// # Examples
/// ```
/// assert!(codetether_relay_okr::is_easy_go_command("/go build feature"));
/// ```
pub fn is_easy_go_command(input: &str) -> bool {
    let command = input
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();

    matches!(command.as_str(), "/go" | "/team")
}
