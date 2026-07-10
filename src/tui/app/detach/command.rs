//! `/detach` — move the current chat thread into a backgrounded sub-agent.
//!
//! The TUI is the main process; nothing leaves it. `/detach` snapshots the
//! current conversation into a [`SpawnedAgent`] that lives in the background
//! of the TUI (alongside `/spawn` agents). The active chat thread is left
//! intact; switch to the detached copy with `Tab` or `/focus <name>`.

use crate::session::Session;
use crate::tui::app::detach::child::build_child;
use crate::tui::app::detach::name::pick_name;
use crate::tui::app::detach::register::register_detached;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

/// Handle `/detach [name]`: snapshot the current thread into a backgrounded
/// sub-agent that keeps running inside the TUI.
pub async fn handle_detach_command(app: &mut App, session: &mut Session, rest: &str) {
    if app.state.processing {
        push(
            app,
            "Cannot detach while a response is in progress. Press Esc first.",
        );
        return;
    }
    if let Err(error) = session.save().await {
        app.state.status = format!("Detach aborted: failed to save session: {error}");
        return;
    }

    let name = pick_name(app, rest);
    let child = match build_child(session).await {
        Ok(c) => c,
        Err(err) => {
            app.state.status = format!("Detach failed: {err}");
            return;
        }
    };
    let child_id = child.id.clone();
    register_detached(app, &name, child);
    app.state.status = format!("Detached thread → subagent '{name}'");
    push(
        app,
        format!(
            "Detached this thread into subagent '{name}' ({}).\n  Open /agents for the all-child dashboard, or switch with Tab / /focus {name}.",
            &child_id[..8.min(child_id.len())]
        ),
    );
}

fn push(app: &mut App, content: impl Into<String>) {
    app.state
        .messages
        .push(ChatMessage::new(MessageType::System, content.into()));
}
