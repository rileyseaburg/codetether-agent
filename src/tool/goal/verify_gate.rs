//! Fail-closed evaluation of a goal transition by a verifier agent.

use super::{Verdict, VerificationRequest, VerifierAgent};

/// Ask `verifier` to judge `request` and return its decision.
///
/// If the verifier errors (cannot start, times out, stops early), the
/// result is [`Verdict::Unavailable`] carrying the error. A broken verifier
/// cannot approve a transition or consume the substantive rejection budget.
///
/// # Arguments
///
/// * `verifier` — The independent agent performing the review.
/// * `request` — The goal requirements and the worker's claim.
///
/// # Returns
///
/// [`Verdict::Pass`] only when the verifier's report ends in `VERDICT: PASS`.
///
/// # Examples
///
/// ```rust
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use codetether_agent::session::tasks::GoalStatus;
/// use codetether_agent::tool::goal::verify::{
///     Verdict, VerificationRequest, VerifierAgent, verify_transition,
/// };
///
/// struct Crashing;
///
/// #[async_trait::async_trait]
/// impl VerifierAgent for Crashing {
///     async fn review(&self, _: &VerificationRequest) -> anyhow::Result<String> {
///         anyhow::bail!("provider unavailable")
///     }
/// }
///
/// let request = VerificationRequest {
///     objective: "Ship".into(), success_criteria: vec![], forbidden: vec![],
///     claimed: GoalStatus::Complete, evidence: String::new(),
/// };
/// let verdict = verify_transition(&Crashing, &request).await;
/// assert!(matches!(verdict, Verdict::Unavailable { findings } if findings.contains("provider unavailable")));
/// # });
/// ```
pub async fn verify_transition(
    verifier: &dyn VerifierAgent,
    request: &VerificationRequest,
) -> Verdict {
    verify_with_identity(verifier, request).await.0
}

pub(crate) async fn verify_with_identity(
    verifier: &dyn VerifierAgent,
    request: &VerificationRequest,
) -> (Verdict, String) {
    let execution = verifier.review_with_identity(request).await;
    let verdict = match execution.report {
        Ok(report) => Verdict::parse(&report),
        Err(error) => Verdict::Unavailable {
            findings: format!("verifier could not reach a decision: {error:#}"),
        },
    };
    (verdict, execution.identity)
}

#[cfg(test)]
#[path = "verify_identity_tests.rs"]
mod identity_tests;
