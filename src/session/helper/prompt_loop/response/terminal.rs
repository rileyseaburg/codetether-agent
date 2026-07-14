//! Final-answer validation and exhausted build-tool retries.

use super::super::super::loop_constants as limits;
use super::super::{Runner, state::StepFlow};
use crate::provider::CompletionResponse;
use anyhow::Result;

/// Rejects exhausted build-mode retries when an execution tool was required.
///
/// # Errors
///
/// Returns an error after the configured retry budget is exhausted.
pub(super) fn ensure_build_calls(runner: &Runner<'_>) -> Result<bool> {
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

/// Accepts a final answer or schedules fixes for validation diagnostics.
///
/// # Errors
///
/// Returns an error when validation fails or its retry budget is exhausted.
pub(super) async fn finish_or_retry(
    runner: &mut Runner<'_>,
    response: CompletionResponse,
) -> Result<StepFlow> {
    runner.session.add_message(response.message);
    if !super::super::super::build::is_build_agent(&runner.session.agent) {
        return Ok(StepFlow::Finish);
    }
    let report = super::super::super::validation::build_validation_report(
        &runner.workspace.cwd,
        &runner.workspace.touched,
        &runner.workspace.baseline_dirty,
    )
    .await?;
    let Some(report) = report else {
        return Ok(StepFlow::Finish);
    };
    runner.progress.validation_retries += 1;
    if runner.progress.validation_retries >= limits::POST_EDIT_VALIDATION_MAX_RETRIES {
        anyhow::bail!(
            "Post-edit validation failed after {} attempts.\n\n{}",
            limits::POST_EDIT_VALIDATION_MAX_RETRIES,
            report.prompt
        );
    }
    super::build_guard::nudge(runner, &report.prompt);
    runner.progress.output.clear();
    Ok(StepFlow::Continue)
}
