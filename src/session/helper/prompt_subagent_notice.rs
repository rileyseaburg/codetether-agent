//! Parent-scoped extraction and durable de-duplication of child results.

use crate::a2a::types::Part;
use crate::bus::{BusEnvelope, BusMessage, global};
use crate::provider::ContentPart;
use crate::session::Session;
use std::collections::HashSet;

pub(super) fn collect(session: &Session, seen: &mut HashSet<String>) -> Option<String> {
    let bus = global()?;
    let topic = format!("agent.{}", session.id);
    collect_from(bus.recorder.recent(1024, Some(&topic)), session, seen)
}

fn collect_from(
    envelopes: Vec<BusEnvelope>,
    session: &Session,
    seen: &mut HashSet<String>,
) -> Option<String> {
    let lines = envelopes
        .into_iter()
        .filter_map(|envelope| extract(envelope, session, seen))
        .collect::<Vec<_>>();
    (!lines.is_empty()).then(|| format!("Sub-agent results:\n{}", lines.join("\n\n")))
}

fn extract(envelope: BusEnvelope, session: &Session, seen: &mut HashSet<String>) -> Option<String> {
    let marker = format!("[sub-agent-event:{}]", envelope.id);
    if !seen.insert(envelope.id) || delivered(session, &marker) {
        return None;
    }
    let BusMessage::AgentMessage { to, parts, .. } = envelope.message else {
        return None;
    };
    if to != session.id {
        return None;
    }
    let text = parts.into_iter().filter_map(|part| match part {
        Part::Text { text } => Some(text),
        Part::File { .. } | Part::Data { .. } => None,
    });
    Some(format!("{marker}\n{}", text.collect::<Vec<_>>().join("\n")))
}

fn delivered(session: &Session, marker: &str) -> bool {
    session
        .history()
        .iter()
        .flat_map(|message| &message.content)
        .any(|part| matches!(part, ContentPart::Text { text } if text.contains(marker)))
}
