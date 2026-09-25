//! Abstraction over the second LLM that reviews goal transitions.

use super::VerificationRequest;
use async_trait::async_trait;

/// An independent agent that reviews a proposed goal transition.
///
/// Implementors return the verifier's raw report. The report must end with
/// a `VERDICT: PASS` or `VERDICT: FAIL` line; see
/// [`Verdict::parse`](super::Verdict::parse).
///
/// # Implementors
///
/// - [`LlmVerifier`](super::LlmVerifier) — production verifier that runs a
///   separate agent loop with workspace verification tools.
///
/// # Examples
///
/// ```rust
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use codetether_agent::session::tasks::GoalStatus;
/// use codetether_agent::tool::goal::verify::{VerificationRequest, VerifierAgent};
///
/// struct Rejecting;
///
/// #[async_trait::async_trait]
/// impl VerifierAgent for Rejecting {
///     async fn review(&self, _: &VerificationRequest) -> anyhow::Result<String> {
///         Ok("FAIL — docs — missing\nVERDICT: FAIL".into())
///     }
/// }
///
/// let request = VerificationRequest {
///     objective: "Write docs".into(), success_criteria: vec![], forbidden: vec![],
///     claimed: GoalStatus::Complete, evidence: String::new(),
/// };
/// let report = Rejecting.review(&request).await.unwrap();
/// assert!(report.ends_with("VERDICT: FAIL"));
/// # });
/// ```
#[async_trait]
pub trait VerifierAgent: Send + Sync {
    /// Review `request` and return the verifier's full report.
    ///
    /// # Errors
    ///
    /// Returns an error when the verifier cannot start or does not finish;
    /// callers treat that as a failed verification.
    async fn review(&self, request: &VerificationRequest) -> anyhow::Result<String>;
}
