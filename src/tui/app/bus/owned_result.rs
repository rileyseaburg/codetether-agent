//! Safe display of managed-child results addressed to the active session.

use crate::a2a::types::Part;
use crate::bus::{BusEnvelope, BusMessage};
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

pub(super) fn render(app: &mut App, envelope: &BusEnvelope) -> bool {
    let BusMessage::AgentMessage { from, to, parts } = &envelope.message else {
        return false;
    };
    let Some(parent) = app.state.session_id.as_deref() else {
        return false;
    };
    if to != parent || !crate::tui::app::managed_agent::owned(parent, from) {
        return false;
    }
    let text = parts
        .iter()
        .filter_map(|part| match part {
            Part::Text { text } => Some(text.as_str()),
            Part::File { .. } | Part::Data { .. } => None,
        })
        .collect::<Vec<_>>()
        .join("\n");
    if !text.trim().is_empty() {
        let sanitized = sanitize(&text);
        let safe = crate::bus::payload::bounded(&sanitized, "\n… [result truncated]");
        app.state
            .messages
            .push(ChatMessage::new(MessageType::Assistant, safe));
        app.state.status = format!("Managed sub-agent @{from} reported back");
        app.state.scroll_to_bottom();
    }
    true
}

fn sanitize(input: &str) -> String {
    input
        .chars()
        .filter(|character| !character.is_control() || matches!(character, '\n' | '\t'))
        .collect()
}
