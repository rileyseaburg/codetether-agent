//! Mux slash-command parsing, execution and human-readable results.
//!
//! Call [run] with the current workspace and input. It recognizes only the
//! `/mux` command, parses its arguments, dispatches through the mux control
//! crates, and returns text for a frontend to present. It owns no TUI state.
mod action;
mod action_helpers;
mod action_parse;
mod execute;
mod format;
#[cfg(test)]
mod tests;
use std::path::Path;

/// Recognize and execute a mux slash command.
///
/// # Arguments
/// * `cwd` — Workspace used for omitted or relative directory arguments.
/// * `prompt` — Complete frontend input, including the command prefix.
/// # Returns
/// `None` for other input, or the mux command's formatted result.
/// # Errors
/// The inner result reports argument, mux startup, storage or control errors.
/// # Examples
/// ```no_run
/// async fn example() {
///     let result = codetether_mux_command::run(std::path::Path::new("."), "/mux help").await;
///     assert!(result.unwrap().unwrap().contains("/mux ls"));
/// }
/// ```
pub async fn run(cwd: &Path, prompt: &str) -> Option<anyhow::Result<String>> {
    let arguments = command_arguments(prompt)?;
    Some(match action_parse::parse(arguments, cwd) {
        Ok(action) => execute::run(action).await,
        Err(error) => Err(anyhow::Error::msg(error)),
    })
}

fn command_arguments(prompt: &str) -> Option<&str> {
    let suffix = prompt.strip_prefix("/mux")?;
    (suffix.is_empty() || suffix.starts_with(char::is_whitespace)).then(|| suffix.trim())
}
