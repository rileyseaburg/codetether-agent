//! Final-answer validation and exhausted build-tool retries.

use super::super::super::loop_constants as limits;
use super::super::{Runner, state::StepFlow};
use crate::provider::CompletionResponse;
use anyhow::Result;

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
    if super::super::super::steering::drain_into(runner.session) > 0 {
        return Ok(StepFlow::Continue);
    }
    if !super::super::super::build::is_build_agent(&runner.session.agent) {
        return Ok(final_flow(runner));
    }
    let report = super::super::super::validation::build_validation_report(
        &runner.workspace.cwd,
        &runner.workspace.touched,
        &runner.workspace.baseline_dirty,
    )
    .await?;
    let Some(report) = report else {
        return Ok(final_flow(runner));
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

fn final_flow(runner: &mut Runner<'_>) -> StepFlow {
    match super::super::super::steering::drain_or_close_into(runner.session) {
        0 => StepFlow::Finish,
        _ => StepFlow::Continue,
    }
}
