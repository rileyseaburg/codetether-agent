//! Heartbeat and progress lifetime for an approved tool invocation.

use super::super::super::super::{tool_exec, tool_policy};
use super::{Call, Runner};

pub(super) async fn run(
    runner: &Runner<'_>,
    call: &Call,
    input: &serde_json::Value,
    started: std::time::Instant,
) -> tool_policy::ToolTuple {
    let host = tool_exec::NativeHost {
        registry: &runner.inner.model.registry,
    };
    codetether_tool_execution::execute_with_heartbeat(
        &host,
        &call.id,
        &call.name,
        input,
        &runner.inner.session.id,
        started,
        runner.inner.events.as_ref(),
    )
    .await
}
