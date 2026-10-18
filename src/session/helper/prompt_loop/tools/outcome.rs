//! Native context adapter for shared tool-outcome processing.
use super::{super::Runner, call::Call};
pub(super) use codetether_tool_outcome::Outcome;
use serde_json::Value;
pub(super) async fn render(
    runner: &mut Runner<'_>,
    call: &Call,
    input: &Value,
    started: std::time::Instant,
    tuple: super::super::super::tool_policy::ToolTuple,
) -> Outcome {
    let context = codetether_tool_outcome::Context {
        tool: &call.name,
        input: &call.input,
        cwd: &runner.inner.workspace.cwd,
        touched: &mut runner.inner.workspace.touched,
        auto_apply_edits: runner.inner.session.metadata.auto_apply_edits,
    };
    codetether_tool_outcome::render(context, input, started, tuple).await
}
