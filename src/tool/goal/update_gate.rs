//! Attempt-capped, audited verification step for `update_goal`.
//!
//! Wraps [`verify_transition`] with two guarantees: every verdict is
//! appended to the session's verdict log, and once a goal hits the
//! rejection cap the verifier stops running and the goal is paused for the
//! user instead of accepting more re-submissions.

use super::attempt_cap::{max_attempts, rejections_since_reset};
use super::verdict_log::{self, VerdictRecord};
use super::verify::{Verdict, VerificationRequest, VerifierAgent, verify_transition};
use crate::session::tasks::Goal;
use crate::tool::ToolResult;
use anyhow::Result;

/// Verify `request`; `Ok(None)` means PASS, `Ok(Some(result))` is a refusal.
///
/// # Errors
///
/// Propagates verdict-log and task-log failures.
pub(super) async fn check(
    session: &str,
    goal: &Goal,
    request: &VerificationRequest,
    verifier: &dyn VerifierAgent,
) -> Result<Option<ToolResult>> {
    let claimed = request.claimed.as_str();
    let cap = max_attempts();
    if rejections_since_reset(&verdict_log::read(session).await?, &goal.id) >= cap {
        let refusal = super::update_escalate::escalate(session, goal, claimed, cap).await?;
        return Ok(Some(refusal));
    }
    let verdict = verify_transition(verifier, request).await;
    let (passed, report) = match &verdict {
        Verdict::Pass => (true, "VERDICT: PASS"),
        Verdict::Fail { findings } => (false, findings.as_str()),
        Verdict::Unavailable { findings } => (false, findings.as_str()),
    };
    let identity = verifier.identity().await;
    let mut record = VerdictRecord::new(&goal.id, claimed, passed, &identity, report);
    record.unavailable = matches!(verdict, Verdict::Unavailable { .. });
    verdict_log::append(session, &record).await?;
    Ok(match verdict {
        Verdict::Pass => None,
        Verdict::Fail { findings } => Some(super::update_reject::result(&findings)),
        Verdict::Unavailable { findings } => Some(ToolResult::error(format!(
            "GOAL_VERIFIER_UNAVAILABLE: no verification decision was reached. Goal state and rejection budget are unchanged. Resolve verifier authentication/runtime failures before retrying; repeating update_goal is not evidence of completion.\n\n{findings}"
        ))),
    })
}
