//! Session creation and registration for `/spawn` (see [`super`]).

#[path = "spawn_agent_message.rs"]
mod message;
#[path = "spawn_agent_register.rs"]
mod register;
use super::SpawnArgs;
use crate::session::Session;
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::models::ViewMode;

/// Push a system note into the chat transcript.
pub(super) fn note(app: &mut App, content: impl Into<String>) {
    app.state
        .messages
        .push(ChatMessage::new(MessageType::System, content.into()));
    app.state.scroll_to_bottom();
}

/// Build the agent session, persist it, and register it in the tree.
pub(super) async fn create_agent(app: &mut App, args: SpawnArgs, depth: u8) {
    let mut session = match Session::new().await {
        Ok(s) => s,
        Err(e) => return fail(app, e),
    };
    session.agent = format!("spawned:{}", args.name);
    session.metadata.auto_apply_edits = true;
    session.add_message(message::system(&args));
    if let Err(e) = session.save().await {
        tracing::warn!(error = %e, "Failed to save spawned agent session");
    }
    let lineage = lineage(args.parent.as_deref(), depth);
    let name = args.name.clone();
    register::insert(app, args, depth, session);
    app.state.status = format!("Deployed subagent: {name}{lineage}");
    app.state.set_view_mode(ViewMode::Subagents);
    note(app, deployment_notice(&name, &lineage));
}

fn fail(app: &mut App, e: anyhow::Error) {
    app.state.status = format!("Failed to create agent session: {e}");
}

fn lineage(parent: Option<&str>, depth: u8) -> String {
    parent
        .map(|p| format!(" under '{p}' (depth {depth})"))
        .unwrap_or_default()
}

fn deployment_notice(name: &str, lineage: &str) -> String {
    format!(
        "Parent deployed subagent '{name}'{lineage}. Open /agents to watch all children and report-back state."
    )
}
