//! Native update workflow with explicitly injected offline verifiers.
use codetether_goal_tools::UpdateArgs;
use codetether_goal_verification::VerifierAgent;
async fn run_with(
    args: UpdateArgs,
    verifier: &dyn VerifierAgent,
) -> anyhow::Result<codetether_tool_core::ToolResult> {
    codetether_goal_tools::update_with::<super::TestHost>(args, verifier).await
}
#[path = "update_run_blocked_tests.rs"]
mod blocked_tests;
#[path = "update_run_cap_tests.rs"]
mod cap_tests;
#[path = "update_run_tests.rs"]
mod tests;
