//! thinking assistant output on the correlated agent bus.
use super::send::send;
use crate::{AgentBus, BusMessage};
use std::sync::Arc;

/// Publish one reasoning segment for the current turn.
/// # Arguments
/// * `bus` — Optional session bus.
/// * `agent`, `turn_id` — Agent identity and parent-turn correlation.
/// * `step` — Provider step for the reasoning event.
/// * `value` — Assistant reasoning.
/// # Returns
/// Publishes when a bus is present, without waiting for subscribers.
/// # Examples
/// ```
/// codetether_bus_core::assistant_events::thinking(None, "build", "turn", 1, "text");
/// ```
pub fn thinking(bus: Option<&Arc<AgentBus>>, agent: &str, turn_id: &str, step: usize, value: &str) {
    send(
        bus,
        agent,
        turn_id,
        format!("agent.{}.thinking", agent),
        BusMessage::AgentThinking {
            agent_id: agent.to_string(),
            thinking: crate::payload::compact_thinking(value),
            step,
        },
    );
}
