//! Slash command prefix boundaries.

/// Match a command only when followed by whitespace or end of input.
/// # Arguments
/// * `input` - Raw command text.
/// * `command` - Exact, case-sensitive command prefix.
/// # Returns
/// The trimmed suffix, or none when the command boundary does not match.
/// # Examples
/// ```
/// assert_eq!(codetether_run_command::command_with_optional_args("/go task", "/go"), Some("task"));
/// assert_eq!(codetether_run_command::command_with_optional_args("/goal", "/go"), None);
/// ```
pub fn command_with_optional_args<'a>(input: &'a str, command: &str) -> Option<&'a str> {
    let trimmed = input.trim();
    let rest = trimmed.strip_prefix(command)?;

    if rest.is_empty() {
        return Some("");
    }

    let first = rest.chars().next()?;
    if first.is_whitespace() {
        Some(rest.trim())
    } else {
        None
    }
}
