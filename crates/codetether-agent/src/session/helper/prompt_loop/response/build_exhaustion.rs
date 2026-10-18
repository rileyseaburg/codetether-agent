//! Native inputs for shared build-retry exhaustion checks.
use super::super::Runner;
use codetether_prompt_loop::response::BuildContext;
pub(super) fn ensure_calls(runner: &Runner<'_>) -> anyhow::Result<bool> {
    runner.inner.progress.ensure_calls(BuildContext {
        agent: &runner.inner.session.agent,
        messages: &runner.inner.session.messages,
        tools: &runner.inner.model.tools,
        cwd: &runner.inner.workspace.cwd,
    })
}
