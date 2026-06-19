//! Maps the high-frequency, non-lifecycle swarm events to bus `SharedResult`
//! messages so observers can read agent progress (decomposition, agent start,
//! tool calls, output, stage completion) by key prefix `swarm/`.

use crate::bus::{BusHandle, BusMessage};
use crate::tui::swarm_view::SwarmEvent;
use serde_json::json;

/// Publish a non-lifecycle event as a tagged `SharedResult`.
pub(super) fn publish_shared(handle: &BusHandle, event: &SwarmEvent) {
    let (key, value) = match event {
        SwarmEvent::Decomposed { subtasks } => (
            "swarm/decomposed".to_string(),
            json!({ "subtasks": subtasks.len() }),
        ),
        SwarmEvent::AgentStarted {
            subtask_id,
            agent_name,
            specialty,
        } => (
            format!("swarm/{subtask_id}/agent"),
            json!({ "agent": agent_name, "specialty": specialty }),
        ),
        SwarmEvent::AgentToolCall {
            subtask_id,
            tool_name,
        } => (
            format!("swarm/{subtask_id}/tool"),
            json!({ "tool": tool_name }),
        ),
        SwarmEvent::AgentOutput { subtask_id, output } => (
            format!("swarm/{subtask_id}/output"),
            json!({ "output": output }),
        ),
        SwarmEvent::StageComplete {
            stage,
            completed,
            failed,
        } => (
            "swarm/stage".to_string(),
            json!({ "stage": stage, "completed": completed, "failed": failed }),
        ),
        // Detail/message events are high-frequency TUI-only refinements.
        _ => return,
    };
    handle.send(
        key.clone(),
        BusMessage::SharedResult {
            key,
            value,
            tags: vec!["swarm".to_string()],
        },
    );
}
