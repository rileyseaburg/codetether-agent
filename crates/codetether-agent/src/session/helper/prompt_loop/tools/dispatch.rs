//! Native entry to ordered shared tool dispatch.
mod host;
use super::super::Runner;
use super::ToolCall;
pub(super) async fn run(
    runner: &mut Runner<'_>,
    step: usize,
    calls: Vec<ToolCall>,
) -> anyhow::Result<()> {
    codetether_tool_parallel::dispatch::run(runner, step, calls).await
}
