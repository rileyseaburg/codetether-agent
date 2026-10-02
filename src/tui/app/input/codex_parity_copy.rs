//! Copy slash commands use message payloads, never rendered terminal rows.

use crate::tui::app::state::App;
use crate::tui::chat::copy;
use crate::tui::clipboard_text::copy_text;

pub(super) fn run(app: &mut App, prompt: &str) {
    let Some(target) = copy::target(prompt) else {
        app.state.status = "Usage: /copy [reply|tool|error]".into();
        app.state.clear_input();
        return;
    };
    app.state.status = match copy::latest(&app.state.messages, target) {
        None => "No matching message to copy.".into(),
        Some(text) => match copy_text(text) {
            Ok(method) => format!("Copied clean text ({method})."),
            Err(error) => {
                tracing::warn!(error = %error, "Copy message failed");
                format!("Could not copy: {error}")
            }
        },
    };
    app.state.clear_input();
}

#[cfg(test)]
#[path = "codex_parity_copy_tests.rs"]
mod tests;
