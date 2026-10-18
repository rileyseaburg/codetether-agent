//! Publish bounded tool output under the subtask identity.
use codetether_bus_core::{AgentBus, BusMessage};
use std::sync::Arc;

/// Publish tool output with its execution outcome and step number.
/// # Arguments
/// Supply the optional bus, subtask identity, step, tool name, output, and success.
/// # Returns
/// Publishes one bounded tool message when a bus is available.
/// # Examples
/// ```
/// codetether_swarm_bus::agent::publish_tool_output(None, "task", 1, "read", "ok", true);
/// ```
pub fn publish_tool_output(
    bus: Option<&Arc<AgentBus>>,
    subtask_id: &str,
    step: usize,
    name: &str,
    output: &str,
    success: bool,
) {
    let Some(bus) = bus else { return };
    bus.handle(subtask_id).send(
        format!("tools.{name}"),
        BusMessage::ToolOutputFull {
            agent_id: subtask_id.to_string(),
            tool_name: name.into(),
            output: crate::tool_output(output),
            success,
            step,
        },
    );
}
