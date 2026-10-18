//! Normalization and pre-execution handling of one tool call.

use super::super::Runner;
use anyhow::Result;
pub(super) use codetether_prompt_loop::Call;
#[path = "driver_host/effects.rs"]
mod effects;
#[path = "driver_host/execute.rs"]
mod execute;
#[path = "driver_host/policy.rs"]
mod policy;

/// Runs one normalized tool call through guards, execution, and publication.
///
/// # Errors
///
/// Returns an error when an execution-stage operation cannot complete.
pub(super) async fn run(runner: &mut Runner<'_>, step: usize, call: Call) -> Result<bool> {
    codetether_prompt_loop::tool_driver::run(runner, step, call).await
}
