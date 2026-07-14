//! Translate overflow recovery into derivation inputs.

use anyhow::Error;

use crate::session::DerivePolicy;

pub(crate) fn resolve(
    error: &Error,
    attempt: usize,
    policy: DerivePolicy,
) -> Option<(DerivePolicy, Option<usize>)> {
    match super::recovery(error, attempt, policy)? {
        super::Plan::Prepared { budget_tokens } => {
            tracing::warn!(%error, budget_tokens, "Retrying with tighter prepared RLM context");
            Some((DerivePolicy::Incremental { budget_tokens }, None))
        }
        super::Plan::Emergency { keep_last } => {
            tracing::warn!(%error, keep_last, "Prepared retry exhausted; applying emergency compaction");
            Some((policy, Some(keep_last)))
        }
    }
}
