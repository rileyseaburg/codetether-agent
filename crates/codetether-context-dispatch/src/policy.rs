//! Apply shared process overrides and reset-marker policy precedence.
use codetether_context_policy::derivation::DerivePolicy;

/// Resolve the effective derivation policy for `session`.
///
/// The persisted session policy is the baseline. Operators can override it
/// process-wide via:
///
/// - `CODETETHER_CONTEXT_POLICY=legacy`
/// - `CODETETHER_CONTEXT_POLICY=reset`
/// - `CODETETHER_CONTEXT_RESET_THRESHOLD_TOKENS=<usize>`
/// - `CODETETHER_CONTEXT_RESET_THRESHOLD_FRACTION=<f64>` (of the model's
///   context window; see [`codetether_context_policy::derivation::reset_threshold`])
///
/// When there is no explicit override and the persisted policy is still
/// legacy, a recorded `[CONTEXT RESET]` marker auto-promotes the effective
/// policy to [`DerivePolicy::Reset`] so `context_reset` markers become live
/// immediately on the next turn.
///
/// `model` scales the Reset threshold to the model's context window so a
/// 1M-context model is not compressed at 3% utilisation.
pub fn effective_policy<H: crate::Source>(session: &H, model: &str) -> DerivePolicy {
    codetether_context_policy::derivation::effective::effective_policy(
        session.saved_policy(),
        codetether_context_legacy::Source::messages(session),
        model,
        |raw| {
            tracing::warn!(target: "codetether_agent::session::context::policy", raw = %raw, "Unknown CODETETHER_CONTEXT_POLICY override; using proactive default");
        },
    )
}
