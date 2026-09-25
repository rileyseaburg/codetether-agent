//! Validation, independent verification, and persistence for `update_goal`.

use super::verify::{LlmVerifier, Verdict, VerificationRequest, VerifierAgent, verify_transition};
use super::{context, response, update::Args};
use crate::session::tasks::GoalStatus;
use crate::tool::ToolResult;
use anyhow::Result;

/// Run `update_goal` with the production LLM verifier.
pub(super) async fn run(args: Args) -> Result<ToolResult> {
    let verifier = LlmVerifier {
        worker_model: args.current_model.clone(),
        workspace: args
            .workspace
            .clone()
            .map_or_else(workspace_default, Into::into),
    };
    run_with(args, &verifier).await
}

/// Apply a terminal goal transition only after `verifier` passes it.
pub(super) async fn run_with(args: Args, verifier: &dyn VerifierAgent) -> Result<ToolResult> {
    let status = match args.status.as_str() {
        "complete" => GoalStatus::Complete,
        "blocked" => GoalStatus::Blocked,
        _ => return Ok(ToolResult::error("status must be complete or blocked")),
    };
    let session_id = context::session_id(args.session_id)?;
    let (_, state) = crate::session::tasks::runtime::current(&session_id).await?;
    let Some(goal) = state.goal else {
        return Ok(ToolResult::error("cannot update goal: no goal exists"));
    };
    let request = VerificationRequest::from_goal(&goal, status, &args.evidence);
    if let Verdict::Fail { findings } = verify_transition(verifier, &request).await {
        tracing::info!(session_id = %session_id, claimed = status.as_str(), "Goal transition rejected by verifier");
        return Ok(super::update_reject::result(&findings));
    }
    crate::session::tasks::runtime::set_status(&session_id, status).await?;
    let (_, state) = crate::session::tasks::runtime::current(&session_id).await?;
    Ok(response::result(&state))
}

fn workspace_default() -> std::path::PathBuf {
    std::env::current_dir().unwrap_or_else(|_| ".".into())
}

#[cfg(test)]
#[path = "update_run_blocked_tests.rs"]
mod blocked_tests;
#[cfg(test)]
#[path = "update_run_tests.rs"]
mod tests;
