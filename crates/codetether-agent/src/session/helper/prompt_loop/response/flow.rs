//! Routing of one normalized completion response.

use super::super::{Runner, state::StepFlow};
use crate::provider::CompletionResponse;
use anyhow::Result;
#[path = "driver_host/effects.rs"]
mod effects;
#[path = "driver_host/guards.rs"]
mod guards;
#[path = "driver_host/tools.rs"]
mod tools;

/// Applies response guards and dispatches any parsed tool calls.
///
/// # Errors
///
/// Returns an error from validation or tool execution.
pub(in crate::session::helper::prompt_loop) async fn handle(
    runner: &mut Runner<'_>,
    step: usize,
    response: CompletionResponse,
) -> Result<StepFlow> {
    codetether_prompt_loop::driver::handle(runner, step, response).await
}
