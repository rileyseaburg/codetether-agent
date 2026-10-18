//! text assistant output on the correlated agent bus.
use super::send::send;
use crate::{AgentBus, BusMessage};
use codetether_a2a_core::Part;
use std::sync::Arc;

/// Publish visible assistant text so a training turn has a completion.
/// # Arguments
/// * `bus` — Optional session bus.
/// * `agent`, `turn_id` — Agent identity and parent-turn correlation.
/// * `value` — Assistant visible text.
/// # Returns
/// Publishes when a bus is present, without waiting for subscribers.
/// # Examples
/// ```
/// codetether_bus_core::assistant_events::text(None, "build", "turn", "text");
/// ```
pub fn text(bus: Option<&Arc<AgentBus>>, agent: &str, turn_id: &str, value: &str) {
    send(
        bus,
        agent,
        turn_id,
        format!("agent.{}.assistant", agent),
        BusMessage::AgentMessage {
            from: agent.to_string(),
            to: "user".into(),
            parts: vec![Part::Text { text: value.into() }],
        },
    );
}
