//! Shell-prefix recognition and submission order.
use crate::View;
use codetether_chat_model::{ChatMessage, MessageType};
use std::path::Path;
/// Consume a shell-prefixed prompt and launch it when the frontend is idle.
/// # Arguments
/// Receives the frontend, working directory and raw composer text.
/// # Returns
/// True for every shell-prefixed prompt, including usage and busy responses.
/// Requires an active Tokio runtime when launching a command.
pub fn submit(view: &mut impl View, cwd: &Path, prompt: &str) -> bool {
    let Some(command) = prompt.strip_prefix('!') else {
        return false;
    };
    let command = command.trim();
    if command.is_empty() {
        view.set_status("Usage: !<shell command>".to_string());
        view.clear_input();
        return true;
    }
    if view.shell().is_running() {
        view.set_status("A shell command is already running.".to_string());
        view.clear_input();
        return true;
    }
    view.push_message(ChatMessage::new(MessageType::User, format!("! {command}")));
    view.set_status(format!("Running shell (background): {command}"));
    view.clear_input();
    view.scroll_to_bottom();
    view.shell().start(cwd, command.to_string());
    true
}
