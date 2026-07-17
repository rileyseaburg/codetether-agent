//! Bridge one read-only group into the shared parallel executor.

use super::super::Runner;
use super::ToolCall;

pub(super) async fn run(runner: &mut Runner<'_>, calls: &[ToolCall]) -> bool {
    let Some(events) = &runner.events else {
        return false;
    };
    super::super::super::tool_parallel::try_execute(
        runner.session,
        calls,
        &runner.model.registry,
        &runner.workspace.cwd,
        &runner.model.model_id,
        runner.model.provider.clone(),
        events,
        &mut runner.progress.codesearch_misses,
    )
    .await
}
