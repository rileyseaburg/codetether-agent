//! Heartbeat and progress lifetime for an approved tool invocation.

use super::super::super::super::{tool_exec, tool_heartbeat, tool_policy};
use super::{Call, Runner};

pub(super) async fn run(
    runner: &Runner<'_>,
    call: &Call,
    input: &serde_json::Value,
    started: std::time::Instant,
) -> tool_policy::ToolTuple {
    let heartbeat = runner
        .events
        .as_ref()
        .map(|events| tool_heartbeat::spawn(events, &call.id, &call.name, started));
    let progress = runner
        .events
        .as_ref()
        .map(|events| (events, call.id.as_str()));
    let result = tool_exec::execute_tool(
        &runner.model.registry,
        &call.name,
        input,
        &runner.session.id,
        started,
        progress,
    )
    .await;
    if let Some(heartbeat) = heartbeat {
        heartbeat.abort();
    }
    result
}
