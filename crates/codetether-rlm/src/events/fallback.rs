//! Subcall fallback event for RLM routing.

use serde::{Deserialize, Serialize};

/// Emitted when a configured subcall model cannot be resolved
/// and the router falls back to the root model.
///
/// This is a cost signal — downstream consumers should log
/// or surface it so the operator knows the subcall tier is
/// misconfigured or unavailable.
///
/// # Examples
///
/// ```rust
/// use codetether_rlm::events::RlmSubcallFallback;
///
/// let fb = RlmSubcallFallback {
///     requested_model: "deepseek-r1".into(),
///     fallback_model: "gpt-4o".into(),
///     reason: "model not found in provider registry".into(),
/// };
/// assert!(!fb.requested_model.is_empty());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RlmSubcallFallback {
    /// The model that was requested for the subcall.
    pub requested_model: String,
    /// The model actually used (typically the root model).
    pub fallback_model: String,
    /// Why the subcall model was unavailable.
    pub reason: String,
}
