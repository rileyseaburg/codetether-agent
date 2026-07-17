//! Exhausted retry handling for build agents that omit tool calls.

use super::super::super::loop_constants as limits;
use super::super::Runner;
use anyhow::Result;

/// Reject an exhausted build-mode retry when execution was required.
///
/// # Errors
///
/// Returns an error after the configured retry budget is exhausted.
pub(super) fn ensure_calls(runner: &Runner<'_>) -> Result<bool> {
    let build = super::super::super::build::is_build_agent(&runner.session.agent);
    let required = super::super::super::build::build_request_requires_tool(
        &runner.session.messages,
        &runner.workspace.cwd,
    );
    if build
        && required
        && runner.progress.build_retries >= limits::BUILD_MODE_TOOL_FIRST_MAX_RETRIES
    {
        anyhow::bail!(
            "Build mode could not obtain tool calls for an explicit file-change request after {} retries. Switch to a tool-capable model and try again.",
            limits::BUILD_MODE_TOOL_FIRST_MAX_RETRIES
        );
    }
    Ok(false)
}
