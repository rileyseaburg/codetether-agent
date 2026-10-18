//! Task-only publication of inbound and outbound A2A server messages.
//!
//! These helpers preserve server correlation without the additional recipient
//! topic used by peer-message publication. Missing buses are a no-op.
use crate::{AgentBus, BusMessage};
use codetether_a2a_core::Message;
#[cfg(test)]
#[path = "a2a_messages_tests.rs"]
mod tests;
/// Publish an inbound message from the remote peer.
///
/// # Arguments
/// Supply an optional bus, local agent name, task id and wire message.
/// # Returns
/// Returns after publication, or immediately when no bus is configured.
/// # Examples
/// ```rust
/// # fn example(message: &codetether_a2a_core::Message) {
/// use codetether_bus_core::a2a_messages::inbound;
/// inbound(None, "local-agent", "task-1", message);
/// # }
/// ```

pub fn inbound(bus: Option<&AgentBus>, agent_name: &str, task_id: &str, message: &Message) {
    emit(bus, task_id, "remote-a2a", agent_name, message);
}
/// Publish an outbound message from the local agent.
///
/// # Arguments
/// Supply an optional bus, local agent name, task id and wire message.
/// # Returns
/// Returns after publication, or immediately when no bus is configured.
/// # Examples
/// ```rust
/// # fn example(message: &codetether_a2a_core::Message) {
/// use codetether_bus_core::a2a_messages::outbound;
/// outbound(None, "local-agent", "task-1", message);
/// # }
/// ```

pub fn outbound(bus: Option<&AgentBus>, agent_name: &str, task_id: &str, message: &Message) {
    emit(bus, task_id, agent_name, "remote-a2a", message);
}
/// Publish one task-correlated A2A message when a bus is available.
///
/// # Arguments
/// Supply an optional bus, task id, sender, recipient and wire message.
/// # Returns
/// Returns after publication, or immediately when no bus is configured.
/// # Examples
/// ```rust
/// # fn example(message: &codetether_a2a_core::Message) {
/// use codetether_bus_core::a2a_messages::emit;
/// emit(None, "task-1", "sender", "recipient", message);
/// # }
/// ```

pub fn emit(bus: Option<&AgentBus>, task_id: &str, from: &str, to: &str, message: &Message) {
    let Some(bus) = bus else {
        return;
    };
    crate::runtime::publication::publish(
        bus,
        "a2a",
        format!("task.{task_id}"),
        BusMessage::AgentMessage {
            from: from.to_string(),
            to: to.to_string(),
            parts: message.parts.clone(),
        },
        Some(task_id.to_string()),
    );
}
