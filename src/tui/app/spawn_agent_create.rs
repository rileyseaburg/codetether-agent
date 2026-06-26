//! Session creation and registration for `/spawn` (see [`super`]).

use super::SpawnArgs;
use crate::session::Session;
use crate::tui::app::state::{App, SpawnedAgent, agent_profile};
use crate::tui::chat::message::{ChatMessage, MessageType};

/// Push a system note into the chat transcript.
pub(super) fn note(app: &mut App, content: impl Into<String>) {
    app.state
        .messages
        .push(ChatMessage::new(MessageType::System, content.into()));
    app.state.scroll_to_bottom();
}

/// System prompt: explicit instructions, else the codename profile blurb.
fn system_prompt(name: &str, instructions: &str) -> String {
    if !instructions.is_empty() {
        return instructions.to_string();
    }
    let p = agent_profile(name);
    format!(
        "You are an AI assistant codenamed '{}' ({}) working as a sub-agent.\n\
         Personality: {}\nCollaboration style: {}\nSignature move: {}",
        p.codename, p.profile, p.personality, p.collaboration_style, p.signature_move,
    )
}

/// Build the agent session, persist it, and register it in the tree.
pub(super) async fn create_agent(app: &mut App, args: SpawnArgs, depth: u8) {
    let mut session = match Session::new().await {
        Ok(s) => s,
        Err(e) => {
            app.state.status = format!("Failed to create agent session: {e}");
            return;
        }
    };
    session.agent = format!("spawned:{}", args.name);
    session.add_message(crate::provider::Message {
        role: crate::provider::Role::System,
        content: vec![crate::provider::ContentPart::Text {
            text: system_prompt(&args.name, &args.instructions),
        }],
    });
    if let Err(e) = session.save().await {
        tracing::warn!(error = %e, "Failed to save spawned agent session");
    }
    let lineage = args
        .parent
        .as_deref()
        .map(|p| format!(" under '{p}' (depth {depth})"))
        .unwrap_or_default();
    app.state.spawned_agents.insert(
        args.name.clone(),
        SpawnedAgent {
            name: args.name.clone(),
            instructions: args.instructions,
            parent: args.parent,
            depth,
            session,
            is_processing: false,
        },
    );
    app.state.status = format!("Spawned agent: {}{lineage}", args.name);
    note(app, format!("Spawned agent '{}'{lineage}.", args.name));
}
