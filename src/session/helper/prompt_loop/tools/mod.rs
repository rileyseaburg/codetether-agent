//! Tool-call batch dispatch for the shared prompt loop.

mod archive;
mod auto_apply;
mod bus;
mod call;
mod call_guard;
mod codesearch;
mod dispatch;
mod invoke;
mod outcome;
mod outcome_detail;
mod parallel;
mod publish;
mod rlm;
mod simple;

use super::Runner;
use anyhow::Result;
type ToolCall = (String, String, serde_json::Value);

/// Executes a parsed tool-call batch, using parallelism for streaming calls.
///
/// # Errors
///
/// Returns an error when sequential tool dispatch cannot complete.
pub(super) async fn execute(
    runner: &mut Runner<'_>,
    step: usize,
    calls: Vec<ToolCall>,
) -> Result<()> {
    dispatch::run(runner, step, calls).await
}
