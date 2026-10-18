//! Usage hints for the first token in a command input.
//! Use CompletionState to manage command candidates without UI dependencies.

/// Find a usage hint for the first whitespace-separated input token.
///
/// # Arguments
/// * `input` — Current composer text.
///
/// # Returns
/// The verbatim command usage hint, or `None` for empty or unknown input.
///
/// # Examples
/// ```
/// assert!(codetether_slash_command::current_slash_hint("/spawn worker").is_some());
/// ```
pub fn current_slash_hint(input: &str) -> Option<&'static str> {
    let cmd = input.split_whitespace().next()?;
    crate::usage_hint(cmd)
}
