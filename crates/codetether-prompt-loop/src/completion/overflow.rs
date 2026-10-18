//! Translate overflow recovery into derivation inputs.

use anyhow::Error;

use codetether_context_policy::{
    derivation::DerivePolicy,
    request::recovery::{Plan, recovery},
};

/// Translate a context-overflow retry plan into derivation inputs.
/// # Arguments
/// `error` identifies the overflow; `attempt` selects the retry stage;
/// `policy` is the attempt's baseline derivation strategy.
/// # Returns
/// The selected policy and optional keep-last override, or `None` if exhausted.
/// # Examples
/// ```
/// use codetether_context_policy::derivation::DerivePolicy;
/// assert!(codetether_prompt_loop::completion::derivation(
///     &anyhow::anyhow!("unrelated error"), 1, DerivePolicy::Legacy,
/// ).is_none());
/// ```
pub fn resolve(
    error: &Error,
    attempt: usize,
    policy: DerivePolicy,
) -> Option<(DerivePolicy, Option<usize>)> {
    match recovery(error, attempt, policy)? {
        Plan::Prepared { budget_tokens } => {
            tracing::warn!(%error, budget_tokens, "Retrying with tighter prepared RLM context");
            Some((DerivePolicy::Incremental { budget_tokens }, None))
        }
        Plan::Emergency { keep_last } => {
            tracing::warn!(%error, keep_last, "Prepared retry exhausted; applying emergency compaction");
            Some((policy, Some(keep_last)))
        }
    }
}
