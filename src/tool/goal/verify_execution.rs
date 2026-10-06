//! A verifier result paired with the identity captured for that execution.

/// One review result with harness-owned identity, not a model self-report.
///
/// # Examples
/// ```
/// use codetether_agent::tool::goal::verify::ReviewExecution;
/// let result = ReviewExecution {
///     report: Ok("VERDICT: PASS".into()),
///     identity: "provider/resolved-model".into(),
/// };
/// assert_eq!(result.identity, "provider/resolved-model");
/// ```
pub struct ReviewExecution {
    /// Raw review text or the runtime failure that prevented a verdict.
    pub report: anyhow::Result<String>,
    /// Provider/model captured for this call, or a custom verifier identifier.
    pub identity: String,
}
