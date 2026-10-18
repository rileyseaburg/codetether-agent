use super::Context;
use crate::{AgentBus, BusMessage};
use std::sync::Arc;
/// Publish a human prompt when a bus is available.
/// # Arguments
/// `bus` is the optional destination, `context` supplies provenance, `message`
/// is original human text and `correlation_id` links it to its turn.
/// # Returns
/// Publishes through the existing bus policy, or returns immediately without a bus.
/// # Examples
/// ```
/// let context = codetether_bus_core::user_prompt::Context { agent: "build", session_id: "1", workspace: None };
/// codetether_bus_core::user_prompt::publish(None, &context, "Hello", None);
/// ```
pub fn publish(
    bus: Option<&Arc<AgentBus>>,
    context: &Context<'_>,
    message: &str,
    correlation_id: Option<String>,
) {
    let Some(bus) = bus else {
        return;
    };
    let workspace = context
        .workspace
        .map(std::path::Path::to_path_buf)
        .or_else(|| std::env::current_dir().ok())
        .map(|p| p.display().to_string())
        .unwrap_or_default();
    let handle = bus.handle(context.agent);
    handle.send_with_correlation(
        format!("agent.{}.user", context.agent),
        BusMessage::UserPrompt {
            agent_id: context.agent.to_owned(),
            text: message.to_string(),
            workspace,
            session_id: context.session_id.to_owned(),
        },
        correlation_id,
    );
}
