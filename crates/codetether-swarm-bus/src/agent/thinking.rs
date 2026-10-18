//! Publish bounded reasoning text under the subtask identity.
use codetether_bus_core::{AgentBus, BusMessage};
use std::sync::Arc;

/// Publish nonempty thinking text to an agent's thinking topic.
/// # Arguments
/// Supply the optional bus, subtask identity, step, and reasoning text.
/// # Returns
/// Publishes one bounded message, or does nothing for empty text/missing bus.
/// # Examples
/// ```
/// codetether_swarm_bus::agent::publish_thinking(None, "task", 1, "thinking");
/// ```
pub fn publish_thinking(
    bus: Option<&Arc<AgentBus>>,
    subtask_id: &str,
    step: usize,
    thinking: &str,
) {
    if thinking.is_empty() {
        return;
    }
    let Some(bus) = bus else { return };
    bus.handle(subtask_id).send(
        format!("agent.{subtask_id}.thinking"),
        BusMessage::AgentThinking {
            agent_id: subtask_id.to_string(),
            thinking: crate::thinking(thinking),
            step,
        },
    );
}
