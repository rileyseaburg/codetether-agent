//! Network mux slash-command dispatch inside the TUI.

mod action;
mod action_helpers;
mod execute;
mod format;
mod output;

use std::path::Path;

use crate::tui::app::state::App;

/// Handle `/mux` commands before a chat session is borrowed.
pub(super) async fn run(app: &mut App, cwd: &Path, prompt: &str) -> bool {
    let Some(arguments) = command_arguments(prompt) else {
        return false;
    };
    let result = match action::parse(arguments, cwd) {
        Ok(action) => execute::run(action).await,
        Err(error) => Err(anyhow::Error::msg(error)),
    };
    output::show(app, result);
    app.state.clear_input();
    true
}

fn command_arguments(prompt: &str) -> Option<&str> {
    let suffix = prompt.strip_prefix("/mux")?;
    (suffix.is_empty() || suffix.starts_with(char::is_whitespace)).then(|| suffix.trim())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
