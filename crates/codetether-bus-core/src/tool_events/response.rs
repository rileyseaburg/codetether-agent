//! Publish correlated compact response and output events.
use super::Context;
use crate::AgentBus;
use std::sync::Arc;
/// Publish the correlated response followed by its compact output event.
/// # Arguments
/// * `bus` — Optional session bus; absence skips publication.
/// * `context` — Tool identity, parent turn and step.
/// * `rendered`, `success` — Rendered output and execution status.
/// # Returns
/// Publishes to the existing agent topics without waiting for subscribers.
/// # Examples
/// ```
/// let context = codetether_bus_core::tool_events::Context { agent: "build", turn_id: "turn", step: 1, request_id: "call", tool_name: "read" };
/// codetether_bus_core::tool_events::response(None, context, "contents", true);
/// ```
pub fn response(bus: Option<&Arc<AgentBus>>, context: Context<'_>, rendered: &str, success: bool) {
    let Some(bus) = bus else {
        return;
    };
    let handle = bus.handle(context.agent);
    let output = crate::payload::compact_tool(rendered);
    handle.send_with_correlation(
        format!("agent.{}.tool.response", context.agent),
        crate::BusMessage::ToolResponse {
            request_id: context.request_id.to_string(),
            agent_id: context.agent.to_string(),
            tool_name: context.tool_name.to_string(),
            result: output.clone(),
            success,
            step: context.step,
        },
        Some(context.turn_id.to_string()),
    );
    handle.send_with_correlation(
        format!("agent.{}.tool.output", context.agent),
        crate::BusMessage::ToolOutputFull {
            agent_id: context.agent.to_string(),
            tool_name: context.tool_name.to_string(),
            output,
            success,
            step: context.step,
        },
        Some(context.turn_id.to_string()),
    );
}
