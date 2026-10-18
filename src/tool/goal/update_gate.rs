//! Native storage binding for shared audited goal verification.
mod host;
use crate::session::tasks::Goal;
use crate::tool::{
    ToolResult,
    goal::verify::{VerificationRequest, VerifierAgent},
};
pub(super) async fn check(
    session: &str,
    goal: &Goal,
    request: &VerificationRequest,
    verifier: &dyn VerifierAgent,
) -> anyhow::Result<Option<ToolResult>> {
    codetether_goal_verification::audit::check::<host::Native>(session, goal, request, verifier)
        .await
}
