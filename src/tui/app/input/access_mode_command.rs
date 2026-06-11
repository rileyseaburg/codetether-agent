use std::path::Path;

use crate::config::{AccessMode, Config};
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

#[path = "access_mode_parse.rs"]
mod parse;
#[path = "access_mode_policy.rs"]
mod policy;

pub(super) async fn run(app: &mut App, cwd: &Path, prompt: &str) -> bool {
    let Some((command, rest)) = parse::prompt(prompt) else {
        return false;
    };
    let rest = rest.trim();
    if rest.is_empty() && command == "/permissions" {
        return false;
    }
    let Some(mode) = parse::mode(rest) else {
        app.state.status = "Usage: /access-mode <ask|approve|full>".to_string();
        return true;
    };
    Config::apply_process_access_mode_override(Some(mode));
    let released = mode == AccessMode::Full && policy::release_active(app);
    let text = format!(
        "Access mode set to `{}`.\n{}",
        parse::label(mode),
        policy::summary(cwd).await
    );
    push(app, with_release_note(text, released));
    app.state.status = format!("Access mode: {}", parse::label(mode));
    app.state.clear_input();
    true
}

fn with_release_note(text: String, released: bool) -> String {
    if released {
        format!("{text}\nApproved the active paused tool request.")
    } else {
        text
    }
}

fn push(app: &mut App, text: String) {
    app.state
        .messages
        .push(ChatMessage::new(MessageType::System, text));
    app.state.scroll_to_bottom();
}
