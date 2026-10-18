//! Validate, audit and commit a requested terminal status.
use super::{Args, Host};
use crate::context;
use anyhow::Result;
use codetether_goal_core::GoalStatus;
use codetether_goal_verification::{VerificationRequest, VerifierAgent};
use codetether_tool_core::ToolResult;
/// Construct the native verifier and request a checked transition.
/// # Arguments
/// * `args` - Requested status, evidence and injected context.
/// # Returns
/// Updated goal JSON or the existing validation/verifier refusal.
/// # Errors
/// Returns runtime, journal or transition errors.
/// # Examples
/// ```no_run
/// async fn run<H: codetether_goal_tools::UpdateHost>(args: codetether_goal_tools::UpdateArgs) -> anyhow::Result<()> {
///     let _result = codetether_goal_tools::update::<H>(args).await?;
///     Ok(())
/// }
/// ```
pub async fn run<H: Host>(args: Args) -> Result<ToolResult> {
    let verifier = H::verifier(&args);
    run_with::<H>(args, &verifier).await
}
/// Apply a transition only after the supplied independent verifier passes it.
/// # Arguments
/// * `args` - Requested status, evidence and injected context.
/// * `verifier` - Independent review implementation.
/// # Returns
/// Updated state or a model-visible refusal.
/// # Errors
/// Returns runtime, journal or transition errors.
/// # Examples
/// ```no_run
/// async fn check<H: codetether_goal_tools::UpdateHost>(args: codetether_goal_tools::UpdateArgs, verifier: &dyn codetether_goal_verification::VerifierAgent) -> anyhow::Result<()> {
///     let _result = codetether_goal_tools::update_with::<H>(args, verifier).await?;
///     Ok(())
/// }
/// ```

/// Apply a terminal goal transition only after `verifier` passes it.
pub async fn run_with<H: Host>(args: Args, verifier: &dyn VerifierAgent) -> Result<ToolResult> {
    let status = match args.status.as_str() {
        "complete" => GoalStatus::Complete,
        "blocked" => GoalStatus::Blocked,
        _ => return Ok(ToolResult::error("status must be complete or blocked")),
    };
    let session_id = context::session_id(args.session_id)?;
    let (_, state) = H::current(&session_id).await?;
    let Some(goal) = state.goal else {
        return Ok(ToolResult::error("cannot update goal: no goal exists"));
    };
    let request = VerificationRequest::from_goal(&goal, status, &args.evidence);
    if goal.status == GoalStatus::Paused {
        return Ok(ToolResult::error(
            "Goal is paused; resume it explicitly before requesting verification.",
        ));
    }
    if let Some(refused) =
        codetether_goal_verification::audit::check::<H>(&session_id, &goal, &request, verifier)
            .await?
    {
        tracing::info!(session_id = %session_id, claimed = status.as_str(), "Goal transition not authorized");
        return Ok(refused);
    }
    super::commit::run::<H>(&session_id, &goal, status).await
}
