//! Native message insertion after the shared tool-first retry decision.
use super::super::super::loop_constants as limits;
use super::super::Runner;
use codetether_prompt_loop::response::BuildContext;
pub(super) fn tool_first(runner: &mut Runner<'_>, text: &str, calls: bool) -> anyhow::Result<bool> {
    let retry = runner.inner.progress.tool_first(
        BuildContext {
            agent: &runner.inner.session.agent,
            messages: &runner.inner.session.messages,
            tools: &runner.inner.model.tools,
            cwd: &runner.inner.workspace.cwd,
        },
        text,
        calls,
    );
    if retry {
        super::nudge::add(runner, limits::BUILD_MODE_TOOL_FIRST_NUDGE);
    }
    Ok(retry)
}
