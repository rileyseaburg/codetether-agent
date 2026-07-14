//! Accumulation and event publication of assistant text.

use super::{preview, state::State};
use crate::tui::swarm_view::{AgentMessageEntry, SwarmEvent};

pub(super) fn record(state: &mut State, text: &str) {
    if text.is_empty() {
        return;
    }
    if !state.output.is_empty() {
        state.output.push('\n');
    }
    state.output.push_str(text);
    tracing::info!(
        step = state.steps,
        output_len = state.output.len(),
        "Sub-agent text output"
    );
    let Some(events) = &state.events else { return };
    let _ = events.try_send(SwarmEvent::AgentMessage {
        subtask_id: state.subtask_id.clone(),
        entry: AgentMessageEntry {
            role: "assistant".into(),
            content: preview::text(text, 500),
            is_tool_call: false,
        },
    });
}
