//! Tool-call batch dispatch for the shared prompt loop.

mod archive;
mod auto_apply;
mod bus;
mod call;
mod call_guard;
mod codesearch;
mod invoke;
mod outcome;
mod outcome_detail;
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
    if let Some(events) = &runner.events {
        if super::super::tool_parallel::try_execute(
            runner.session,
            &calls,
            &runner.model.registry,
            &runner.workspace.cwd,
            &runner.model.model_id,
            runner.model.provider.clone(),
            events,
            &mut runner.progress.codesearch_misses,
        )
        .await
        {
            return Ok(());
        }
    }
    for (id, name, input) in calls {
        let call = call::Call::new(id, name, input);
        if call::run(runner, step, call).await? {
            break;
        }
    }
    Ok(())
}
