//! Safe display of managed-child results addressed to the active session.

use super::Host;
use codetether_a2a_core::Part;
use codetether_bus_core::{BusEnvelope, BusMessage};

pub(super) fn render(host: &mut impl Host, envelope: &BusEnvelope) -> bool {
    let BusMessage::AgentMessage { from, to, parts } = &envelope.message else {
        return false;
    };
    let Some(parent) = host.session_id() else {
        return false;
    };
    if to != parent || !host.owns_agent(parent, from) {
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
        let safe = codetether_bus_core::payload::bounded(&sanitized, "\n… [result truncated]");
        host.push_assistant(safe);
        host.set_status(format!("Managed sub-agent @{from} reported back"));
        host.scroll_to_bottom();
    }
    true
}

fn sanitize(input: &str) -> String {
    input
        .chars()
        .filter(|character| !character.is_control() || matches!(character, '\n' | '\t'))
        .collect()
}
