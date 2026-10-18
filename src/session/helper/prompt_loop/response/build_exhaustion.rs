//! Native inputs for shared build-retry exhaustion checks.
use super::super::Runner;
use codetether_prompt_loop::response::BuildContext;
pub(super) fn ensure_calls(runner: &Runner<'_>) -> anyhow::Result<bool> {
    runner.progress.ensure_calls(BuildContext {
        agent: &runner.session.agent,
        messages: &runner.session.messages,
        tools: &runner.model.tools,
        cwd: &runner.workspace.cwd,
    })
}
