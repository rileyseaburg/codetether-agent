//! Policy dispatcher — routes to the chosen [`DerivePolicy`] implementation.

use crate::session::Session;
use crate::session::derive_policy::DerivePolicy;

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
pub fn effective_policy(session: &Session, model: &str) -> DerivePolicy {
    codetether_context_dispatch::effective_policy(session, model)
}

pub use super::dispatch::derive_with_policy;
