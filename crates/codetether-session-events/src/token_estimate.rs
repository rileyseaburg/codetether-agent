//! Shared TokenEstimate event payload and operations.
use serde::{Deserialize, Serialize};

/// Pre-flight estimate of the next request's token footprint.
///
/// Emitted before the provider is contacted so the TUI can render
/// budget-aware warnings ("73 % of 128k window") and the compaction
/// pipeline can decide whether to run a pre-emptive summary pass.
///
/// The `budget` field is the usable window after the implementation has
/// subtracted its completion reserve and protocol overhead — it is **not**
/// the raw context window from the model card.
///
/// # Examples
///
/// ```rust
/// use codetether_session_events::TokenEstimate;
///
/// let est = TokenEstimate {
///     model: "anthropic/claude-opus-4-7".into(),
///     request_tokens: 94_000,
///     budget: 128_000,
/// };
/// assert!(est.fraction() > 0.7);
/// assert!(!est.is_over_budget());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenEstimate {
    /// Provider-qualified model identifier the estimate targets.
    pub model: String,
    /// Estimated total request tokens (system + messages + tools).
    pub request_tokens: usize,
    /// Usable token budget for the request after reserves are subtracted.
    pub budget: usize,
}

impl TokenEstimate {
    /// Fraction of the budget the request is projected to consume.
    ///
    /// Returns `0.0` when `budget == 0` rather than panicking, so callers
    /// may feed the value directly into UI colour ramps.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_session_events::TokenEstimate;
    ///
    /// let e = TokenEstimate { model: "m".into(), request_tokens: 50, budget: 100 };
    /// assert!((e.fraction() - 0.5).abs() < 1e-9);
    ///
    /// let z = TokenEstimate { model: "m".into(), request_tokens: 10, budget: 0 };
    /// assert_eq!(z.fraction(), 0.0);
    /// ```
    pub fn fraction(&self) -> f64 {
        if self.budget == 0 {
            0.0
        } else {
            self.request_tokens as f64 / self.budget as f64
        }
    }

    /// Returns `true` when the estimate strictly exceeds the usable budget.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_session_events::TokenEstimate;
    ///
    /// let e = TokenEstimate { model: "m".into(), request_tokens: 129_000, budget: 128_000 };
    /// assert!(e.is_over_budget());
    /// ```
    pub fn is_over_budget(&self) -> bool {
        self.request_tokens > self.budget
    }
}
