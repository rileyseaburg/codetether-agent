//! `!command` prefix handling for chat submit.
//!
//! Lines starting with `!` are executed as shell commands in the
//! workspace directory instead of being sent to the model. The command
//! and its output are appended to the chat history as a tool-style
//! message so the transcript stays readable.

use std::path::Path;

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

use super::exec::run_shell;

/// Handle a `!command` shell invocation. Returns `true` when the
/// prompt was consumed as a shell command.
pub(in crate::tui::app::input) async fn run(app: &mut App, cwd: &Path, prompt: &str) -> bool {
    let Some(command) = prompt.strip_prefix('!') else {
        return false;
    };
    let command = command.trim();
    if command.is_empty() {
        app.state.status = "Usage: !<shell command>".to_string();
        app.state.clear_input();
        return true;
    }
    app.state
        .messages
        .push(ChatMessage::new(MessageType::User, format!("! {command}")));
    app.state.status = format!("Running shell: {command}");
    app.state.clear_input();
    let outcome = run_shell(command, cwd).await;
    app.state.messages.push(ChatMessage::new(
        MessageType::ToolResult {
            name: "shell".to_string(),
            output: outcome.output.clone(),
            success: outcome.success,
            duration_ms: Some(outcome.duration_ms),
        },
        outcome.output,
    ));
    app.state.status = if outcome.success {
        format!("Shell command finished ({} ms)", outcome.duration_ms)
    } else {
        format!("Shell command failed ({} ms)", outcome.duration_ms)
    };
    app.state.scroll_to_bottom();
    true
}
