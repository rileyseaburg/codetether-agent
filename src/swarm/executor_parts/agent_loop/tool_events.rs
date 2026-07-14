//! Real-time events for sub-agent tool calls.

use super::{preview, state::State};
use crate::tui::swarm_view::{AgentToolCallDetail, SwarmEvent};

pub(super) fn started(state: &State, name: &str) {
    if let Some(events) = &state.events {
        let _ = events.try_send(SwarmEvent::AgentToolCall {
            subtask_id: state.subtask_id.clone(),
            tool_name: name.into(),
        });
    }
}

pub(super) fn finished(state: &State, name: &str, input: &str, output: &str, success: bool) {
    if let Some(events) = &state.events {
        let _ = events.try_send(SwarmEvent::AgentToolCallDetail {
            subtask_id: state.subtask_id.clone(),
            detail: AgentToolCallDetail {
                tool_name: name.into(),
                input_preview: preview::text(input, 200),
                output_preview: preview::text(output, 500),
                success,
            },
        });
    }
}
