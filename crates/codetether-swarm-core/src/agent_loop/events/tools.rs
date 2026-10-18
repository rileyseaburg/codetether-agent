//! Nonblocking publication of sub-agent tool-call activity.
use super::preview;
use crate::SwarmEvent;
use codetether_agent_core::events::AgentToolCallDetail;
use tokio::sync::mpsc;

/// Publish the start of a sub-agent tool call when events are enabled.
/// # Arguments
/// Supply the optional sender, subtask identity, and tool name.
/// # Returns
/// Returns after attempting nonblocking delivery; full/closed channels are ignored.
/// # Examples
/// ```
/// codetether_swarm_core::agent_loop::events::started(None, "task", "read");
/// ```
pub fn started(events: Option<&mpsc::Sender<SwarmEvent>>, subtask_id: &str, name: &str) {
    if let Some(events) = events {
        let _ = events.try_send(SwarmEvent::AgentToolCall {
            subtask_id: subtask_id.to_string(),
            tool_name: name.into(),
        });
    }
}

/// Publish bounded input/output previews for a completed tool call.
/// # Arguments
/// Supply the optional sender, subtask identity, tool name, raw input/output,
/// and whether execution succeeded.
/// # Returns
/// Returns after attempting nonblocking delivery, preserving 200/500-byte limits.
/// # Examples
/// ```
/// codetether_swarm_core::agent_loop::events::finished(None, "task", "read", "{}", "ok", true);
/// ```
pub fn finished(
    events: Option<&mpsc::Sender<SwarmEvent>>,
    subtask_id: &str,
    name: &str,
    input: &str,
    output: &str,
    success: bool,
) {
    if let Some(events) = events {
        let _ = events.try_send(SwarmEvent::AgentToolCallDetail {
            subtask_id: subtask_id.to_string(),
            detail: AgentToolCallDetail {
                tool_name: name.into(),
                input_preview: preview(input, 200),
                output_preview: preview(output, 500),
                success,
            },
        });
    }
}
