//! Assistant output accumulation and live event publication.
use super::preview;
use crate::SwarmEvent;
use codetether_agent_core::events::AgentMessageEntry;
use tokio::sync::mpsc;

/// Append assistant text and publish its bounded event preview.
/// # Arguments
/// Supply the output buffer, step number, optional sender, subtask identity,
/// and assistant text to append.
/// # Returns
/// Empty text does nothing. Other text is appended with an inter-message newline
/// and offered to the event channel without blocking.
/// # Examples
/// ```
/// let mut output = "first".to_string();
/// codetether_swarm_core::agent_loop::events::record_output(&mut output, 1, None, "task", "second");
/// assert_eq!(output, "first\nsecond");
/// ```
pub fn record_output(
    output: &mut String,
    step: usize,
    events: Option<&mpsc::Sender<SwarmEvent>>,
    subtask_id: &str,
    text: &str,
) {
    if text.is_empty() {
        return;
    }
    if !output.is_empty() {
        output.push('\n');
    }
    output.push_str(text);
    tracing::info!(
        target: "codetether_agent::swarm::executor::agent_loop::output",
        step = step,
        output_len = output.len(),
        "Sub-agent text output"
    );
    let Some(events) = events else { return };
    let _ = events.try_send(SwarmEvent::AgentMessage {
        subtask_id: subtask_id.to_string(),
        entry: AgentMessageEntry {
            role: "assistant".into(),
            content: preview(text, 500),
            is_tool_call: false,
        },
    });
}
