//! Bridge one read-only group into the shared parallel executor.

use super::super::Runner;
use super::ToolCall;

pub(super) async fn run(runner: &mut Runner<'_>, calls: &[ToolCall]) -> bool {
    let Some(events) = &runner.inner.events else {
        return false;
    };
    super::super::super::tool_parallel::try_execute(
        runner.inner.session,
        calls,
        &runner.inner.model.registry,
        &runner.inner.workspace.cwd,
        &runner.inner.model.model_id,
        runner.inner.model.provider.clone(),
        events,
        &mut runner.inner.progress.codesearch_misses,
    )
    .await
}
