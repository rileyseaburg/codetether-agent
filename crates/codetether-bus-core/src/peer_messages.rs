//! Correlated A2A peer messages on the local bus.
use std::sync::Arc;

use crate::{AgentBus, BusMessage};
use codetether_a2a_core::Message;

/// Publish a peer message to its correlated task and optional inbound agent topic.
///
/// # Arguments
/// * `bus` - Bus receiving the message.
/// * `task_id` - Task topic and correlation identifier.
/// * `from` - Sender name; remote-a2a also routes to the recipient's A2A topic.
/// * `to` - Recipient name.
/// * `message` - Wire message whose content parts are copied into the bus event.
/// # Returns
/// Unit after sending the correlated events.
/// # Examples
/// ```rust,no_run
/// # fn example(bus: &std::sync::Arc<codetether_bus_core::AgentBus>, message: &codetether_a2a_core::Message) {
/// codetether_bus_core::peer_messages::emit(bus, "task", "remote-a2a", "agent", message);
/// # }
/// ```
pub fn emit(bus: &Arc<AgentBus>, task_id: &str, from: &str, to: &str, message: &Message) {
    let payload = BusMessage::AgentMessage {
        from: from.to_string(),
        to: to.to_string(),
        parts: message.parts.clone(),
    };
    let handle = bus.handle("a2a");
    handle.send_with_correlation(
        format!("task.{task_id}"),
        payload.clone(),
        Some(task_id.into()),
    );
    if from == "remote-a2a" {
        handle.send_with_correlation(format!("a2a.{to}"), payload, Some(task_id.into()));
    }
}
