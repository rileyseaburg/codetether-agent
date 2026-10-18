//! Normalization and pre-execution handling of one tool call.

use super::super::Runner;
use anyhow::Result;
pub(super) use codetether_prompt_loop::Call;

/// Runs one normalized tool call through guards, execution, and publication.
///
/// # Errors
///
/// Returns an error when an execution-stage operation cannot complete.
pub(super) async fn run(runner: &mut Runner<'_>, step: usize, call: Call) -> Result<bool> {
    super::call_guard::publish_start(runner, &call).await;
    if call.name == "list_tools" {
        return super::discovery::run(runner, &call).await;
    }
    super::bus::request(runner, step, &call);
    if let Some(reason) = super::call_guard::blocked(runner, &call) {
        super::simple::record(runner, &call, format!("Error: {reason}"), false).await;
        return Ok(false);
    }
    let outcome = super::invoke::execute(runner, &call).await?;
    runner
        .progress
        .decline_guard
        .record(&call.name, &call.input, outcome.metadata.as_ref());
    super::refresh::after(runner, &call, outcome.success).await?;
    super::publish::complete(runner, step, &call, outcome).await?;
    Ok(super::codesearch::guard(runner, step))
}
