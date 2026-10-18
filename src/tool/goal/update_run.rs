//! Native specialization used by goal-transition integration tests.
use super::{host::Native, update::Args, verify::VerifierAgent};
use crate::tool::ToolResult;
/// Resolve the native verifier and execute the shared update workflow.
pub(super) async fn run(args: Args) -> anyhow::Result<ToolResult> {
    codetether_goal_tools::update::<Native>(args).await
}
/// Apply a terminal goal transition through shared verified execution.
pub(super) async fn run_with(
    args: Args,
    verifier: &dyn VerifierAgent,
) -> anyhow::Result<ToolResult> {
    codetether_goal_tools::update_with::<Native>(args, verifier).await
}

#[cfg(test)]
#[path = "update_run_blocked_tests.rs"]
mod blocked_tests;
#[cfg(test)]
#[path = "update_run_cap_tests.rs"]
mod cap_tests;
#[cfg(test)]
#[path = "update_run_tests.rs"]
mod tests;
