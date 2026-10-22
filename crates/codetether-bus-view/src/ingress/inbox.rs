use super::Host;
use codetether_a2a_core::Part;
use codetether_bus_core::{BusEnvelope, BusMessage};
use codetether_worker_events::IncomingTask;

/// Apply recorded bus input through frontend bindings.
/// # Arguments
/// * `host` — Frontend identities, observers and presentation effects.
/// * Input — Recorded envelope, message or recorder to process.
/// # Returns
/// Updates only the state selected by the bus message and routing rules.
/// # Examples
/// ```no_run
/// # fn example(host: &mut impl codetether_bus_view::ingress::Host, envelope: &codetether_bus_core::BusEnvelope) {
/// codetether_bus_view::ingress::maybe_queue(host, envelope);
/// # }
/// ```
pub fn maybe_queue(host: &mut impl Host, envelope: &BusEnvelope) {
    if super::owned_result::render(host, envelope) {
        return;
    }
    let BusMessage::AgentMessage { from, to, parts } = &envelope.message else {
        return;
    };
    if should_ignore(host, envelope, from, to) {
        return;
    }
    if host.session_id() == Some(to) && !host.knows_peer(to, from) {
        return;
    }
    let message = text_parts(parts);
    if !message.trim().is_empty() {
        host.enqueue_task(IncomingTask {
            task_id: envelope
                .correlation_id
                .clone()
                .unwrap_or_else(|| envelope.id.clone()),
            message: format!(
                "[sub-agent-event:{}]\n{}",
                envelope.id,
                super::prompt::untrusted(from, &message)
            ),
            from_agent: Some(from.clone()),
        });
    }
}

fn should_ignore(host: &impl Host, envelope: &BusEnvelope, from: &str, to: &str) -> bool {
    !envelope.topic.starts_with("agent.")
        || from == to
        || matches!(from, "a2a" | "a2a-stream")
        || !addressed_to_tui(host, to)
}

fn addressed_to_tui(host: &impl Host, to: &str) -> bool {
    to == "tui"
        || host.worker_id() == Some(to)
        || host.is_registered(to)
        || host.session_id() == Some(to)
}

fn text_parts(parts: &[Part]) -> String {
    parts
        .iter()
        .filter_map(|part| match part {
            Part::Text { text } => Some(text.as_str()),
            Part::File { .. } | Part::Data { .. } => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}
