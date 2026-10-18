//! Message filtering and durable transcript duplicate suppression.
use super::Host;
use codetether_a2a_core::Part;
use codetether_bus_core::{BusEnvelope, BusMessage};
use codetether_provider_core::ContentPart;
use std::collections::HashSet;

pub(super) fn extract(
    envelope: BusEnvelope,
    session: &impl Host,
    seen: &mut HashSet<String>,
) -> Option<String> {
    let marker = format!("[sub-agent-event:{}]", envelope.id);
    if !seen.insert(envelope.id) || delivered(session, &marker) {
        return None;
    }
    let BusMessage::AgentMessage { to, parts, .. } = envelope.message else {
        return None;
    };
    if to != session.session_id() {
        return None;
    }
    let text = parts.into_iter().filter_map(|part| match part {
        Part::Text { text } => Some(text),
        Part::File { .. } | Part::Data { .. } => None,
    });
    Some(format!("{marker}\n{}", text.collect::<Vec<_>>().join("\n")))
}

fn delivered(session: &impl Host, marker: &str) -> bool {
    session
        .history()
        .iter()
        .flat_map(|message| &message.content)
        .any(|part| matches!(part, ContentPart::Text { text } if text.contains(marker)))
}
