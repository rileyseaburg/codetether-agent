//! `!command` prefix handling for chat submit.
//!
//! Lines starting with `!` are executed as shell commands in the
//! workspace directory instead of being sent to the model. The command
//! runs on a background task (see [`crate::tui::app::input::shell_bg`]) so
//! the TUI stays responsive during long-running commands; its result is
//! drained into the transcript by the event loop.

use std::path::Path;

use crate::tui::app::input::shell_bg::spawn_shell_command;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

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
    if app.state.shell_running {
        app.state.status = "A shell command is already running.".to_string();
        app.state.clear_input();
        return true;
    }
    app.state
        .messages
        .push(ChatMessage::new(MessageType::User, format!("! {command}")));
    app.state.status = format!("Running shell (background): {command}");
    app.state.clear_input();
    app.state.scroll_to_bottom();
    spawn_shell_command(app, cwd, command.to_string());
    true
}
