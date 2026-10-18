//! Approval-aware execution of one normalized tool call.

#[path = "invoke_approved.rs"]
mod approved;
use super::{super::Runner, call::Call, outcome::Outcome};
#[path = "invoke_host.rs"]
mod host;

/// Applies approval policy and executes a normalized tool call.
pub(super) async fn execute(runner: &mut Runner<'_>, call: &Call) -> anyhow::Result<Outcome> {
    codetether_prompt_loop::tool_driver::invocation::execute(runner, call).await
}
