//! Context-window usage ratios.
//!
//! The TUI uses this to render the "context %" badge next to the model name.

use serde::{Deserialize, Serialize};

/// How much of a model's context window has been consumed.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::telemetry::ContextLimit;
///
/// let cl = ContextLimit::new(50_000, 200_000);
/// assert_eq!(cl.remaining_tokens, 150_000);
/// assert!((cl.percentage() - 25.0).abs() < 1e-6);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextLimit {
    /// Model's total context window in tokens.
    pub max_tokens: u64,
    /// Tokens currently in use.
    pub used_tokens: u64,
    /// `max_tokens - used_tokens`, saturating at zero.
    pub remaining_tokens: u64,
    /// `used_tokens / max_tokens * 100`.
    pub percentage_used: f64,
    /// Alias for [`Self::percentage_used`] retained for API compatibility.
    pub percentage: f64,
}

impl ContextLimit {
    /// Construct from raw counts. `max_tokens == 0` is handled by reporting
    /// `0.0` percentage rather than dividing by zero.
    pub fn new(used_tokens: u64, max_tokens: u64) -> Self {
        let remaining = max_tokens.saturating_sub(used_tokens);
        let percentage = if max_tokens > 0 {
            (used_tokens as f64 / max_tokens as f64) * 100.0
        } else {
            0.0
        };
        Self {
            max_tokens,
            used_tokens,
            remaining_tokens: remaining,
            percentage_used: percentage,
            percentage,
        }
    }

    /// Accessor kept for API compatibility; returns [`Self::percentage_used`].
    pub fn percentage(&self) -> f64 {
        self.percentage_used
    }
}
