//! Read optional arguments after an exact command name.
//! Use the exported function to process command input without UI state.

/// Read optional arguments after an exact command name.
///
/// # Arguments
/// * `input` — Command input to inspect.
///
/// # Returns
/// Arguments when the command boundary matches, otherwise `None`.
///
/// # Examples
/// ```
/// assert_eq!(codetether_slash_command::command_with_optional_args("/file a", "/file"), Some("a"));
/// ```
/// The `command` argument supplies the full command name to match.
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
