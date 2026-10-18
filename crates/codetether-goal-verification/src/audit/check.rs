//! Attempt-capped, audited verification step for `update_goal`.
//!
//! Wraps [`crate::verify_transition`] with two guarantees: every verdict is
//! appended to the session's verdict log, and once a goal hits the
//! rejection cap the verifier stops running and the goal is paused for the
//! user instead of accepting more re-submissions.

use super::Host;
use crate::VerdictRecord;
use crate::attempt_cap::{max_attempts, rejections_since_reset};
use crate::{Verdict, VerificationRequest, VerifierAgent, verify_with_identity};
use anyhow::Result;
use codetether_goal_core::Goal;
use codetether_tool_core::ToolResult;

/// Verify `request`; `Ok(None)` means PASS, `Ok(Some(result))` is a refusal.
///
/// # Errors
///
/// Propagates verdict-log and task-log failures.
/// # Arguments
/// * `session` - Owning session identifier.
/// * `goal` - Persisted goal revision being reviewed.
/// * `request` - Requirements and claimed transition.
/// * `verifier` - Independent review implementation.
/// # Returns
/// None for a passing verdict; a model-visible refusal otherwise.
/// # Examples
/// ```no_run
/// async fn check<H: codetether_goal_verification::audit::Host>(goal: &codetether_goal_core::Goal, request: &codetether_goal_verification::VerificationRequest, verifier: &dyn codetether_goal_verification::VerifierAgent) {
///     let _outcome = codetether_goal_verification::audit::check::<H>("session", goal, request, verifier).await;
/// }
/// ```
pub async fn check<H: Host>(
    session: &str,
    goal: &Goal,
    request: &VerificationRequest,
    verifier: &dyn VerifierAgent,
) -> Result<Option<ToolResult>> {
    let claimed = request.claimed.as_str();
    let cap = max_attempts();
    if rejections_since_reset(&H::read(session).await?, &goal.id) >= cap {
        let refusal = super::escalate::escalate::<H>(session, goal, claimed, cap).await?;
        return Ok(Some(refusal));
    }
    let (verdict, identity) = verify_with_identity(verifier, request).await;
    let (passed, report) = match &verdict {
        Verdict::Pass => (true, "VERDICT: PASS"),
        Verdict::Fail { findings } => (false, findings.as_str()),
        Verdict::Unavailable { findings } => (false, findings.as_str()),
    };
    let mut record = VerdictRecord::new(&goal.id, claimed, passed, &identity, report);
    record.goal_updated_at = Some(goal.last_updated_at);
    record.unavailable = matches!(verdict, Verdict::Unavailable { .. });
    H::append(session, &record).await?;
    Ok(match verdict {
        Verdict::Pass => None,
        Verdict::Fail { findings } => Some(super::reject::result(&findings)),
        Verdict::Unavailable { findings } => Some(ToolResult::error(format!(
            "GOAL_VERIFIER_UNAVAILABLE: no verification decision was reached. Goal state and rejection budget are unchanged. Resolve verifier authentication/runtime failures before retrying; repeating update_goal is not evidence of completion.\n\n{findings}"
        ))),
    })
}
