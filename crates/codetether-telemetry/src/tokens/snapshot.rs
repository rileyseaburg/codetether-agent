//! Read-only snapshots of token usage.
//!
//! Snapshots are plain data — they don't hold locks and can be freely cloned
//! across threads or serialized for the TUI / API.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::totals::TokenTotals;

/// A point-in-time view of the global [`super::AtomicTokenCounter`].
///
/// # Examples
///
/// ```rust
/// use codetether_telemetry::GlobalTokenSnapshot;
///
/// let s = GlobalTokenSnapshot::new(1_000, 500, 1_500);
/// assert_eq!(s.totals.total(), 1_500);
/// assert!(s.summary().contains("1500"));
/// ```
#[derive(Debug, Clone, Default)]
pub struct GlobalTokenSnapshot {
    /// Cumulative prompt tokens.
    pub input: u64,
    /// Cumulative completion tokens.
    pub output: u64,
    /// Deprecated duplicate of [`Self::totals`] kept for API compatibility.
    pub total: TokenTotals,
    /// Preferred aggregate view.
    pub totals: TokenTotals,
    /// Number of completions recorded since startup.
    pub request_count: u64,
}

impl GlobalTokenSnapshot {
    /// Build a snapshot from raw counts (the third arg is ignored and kept
    /// for API compatibility).
    pub fn new(input: u64, output: u64, _total: u64) -> Self {
        Self {
            input,
            output,
            total: TokenTotals::new(input, output),
            totals: TokenTotals::new(input, output),
            request_count: 0,
        }
    }

    /// Human-readable one-liner, e.g. `"1500 total tokens (1000 input, 500 output)"`.
    pub fn summary(&self) -> String {
        format!(
            "{} total tokens ({} input, {} output)",
            self.totals.total(),
            self.input,
            self.output
        )
    }
}
mod model;
pub use model::TokenUsageSnapshot;
