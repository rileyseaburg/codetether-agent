//! Event loop handler for autochat UI events.

use crate::tui::app::state::AppState;
use crate::tui::chat::message::{ChatMessage, MessageType};

use super::events::AutochatUiEvent;

/// Process a single autochat UI event, updating app state.
pub fn handle_autochat_event(app: &mut AppState, event: AutochatUiEvent) {
    match event {
        AutochatUiEvent::Progress(msg) => {
            app.status = msg;
        }
        AutochatUiEvent::SystemMessage(msg) => {
            app.messages
                .push(ChatMessage::new(MessageType::System, msg));
            app.scroll_to_bottom();
        }
        AutochatUiEvent::AgentEvent {
            agent_name,
            event: _,
        } => {
            app.status = format!("🔄 {agent_name} processing…");
        }
        AutochatUiEvent::Completed {
            summary,
            okr_id,
            okr_run_id,
            relay_id,
        } => {
            app.autochat.running = false;
            let mut msg = format!("✅ Autochat relay completed: {summary}");
            if let Some(oid) = okr_id {
                msg.push_str(&format!("\nOKR: {oid}"));
            }
            if let Some(rid) = relay_id {
                msg.push_str(&format!("\nRelay: {rid}"));
            }
            let _ = okr_run_id;
            app.messages
                .push(ChatMessage::new(MessageType::System, msg));
            app.scroll_to_bottom();
        }
    }
}
