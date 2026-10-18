//! Publish a correlated tool request.
use super::Context;
use crate::AgentBus;
use serde_json::Value;
use std::sync::Arc;
/// Publish one correlated tool request.
/// # Arguments
/// * `bus` — Optional session bus; absence skips publication.
/// * `context` — Tool identity, parent turn and step.
/// * `input` — Original tool arguments.
/// # Returns
/// Publishes to the existing agent topics without waiting for subscribers.
/// # Examples
/// ```
/// let context = codetether_bus_core::tool_events::Context { agent: "build", turn_id: "turn", step: 1, request_id: "call", tool_name: "read" };
/// codetether_bus_core::tool_events::request(None, context, &serde_json::json!({}));
/// ```
pub fn request(bus: Option<&Arc<AgentBus>>, context: Context<'_>, input: &Value) {
    let Some(bus) = bus else {
        return;
    };
    bus.handle(context.agent).send_with_correlation(
        format!("agent.{}.tool.request", context.agent),
        crate::BusMessage::ToolRequest {
            request_id: context.request_id.to_string(),
            agent_id: context.agent.to_string(),
            tool_name: context.tool_name.to_string(),
            arguments: input.clone(),
            step: context.step,
        },
        Some(context.turn_id.to_string()),
    );
}
