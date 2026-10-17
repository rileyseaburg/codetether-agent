//! Per-model token snapshots and current process usage.

use super::*;

/// Per-model token snapshot. Instances are stamped with [`Utc::now`] when
/// produced by [`super::super::AtomicTokenCounter::model_snapshots`].
///
/// # Examples
///
/// ```rust
/// use codetether_telemetry::TokenUsageSnapshot;
///
/// // `current()` pulls from the global counter and never panics.
/// let s = TokenUsageSnapshot::current();
/// assert_eq!(s.name, "global");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsageSnapshot {
    /// Scope name (model id, or `"global"` for the whole process).
    pub name: String,
    /// Prompt tokens observed in this scope.
    pub prompt_tokens: u64,
    /// Completion tokens observed in this scope.
    pub completion_tokens: u64,
    /// `prompt_tokens + completion_tokens`.
    pub total_tokens: u64,
    /// Convenience aggregate.
    pub totals: TokenTotals,
    /// When this snapshot was produced.
    pub timestamp: DateTime<Utc>,
    /// Number of requests rolled into this snapshot (0 if unknown).
    pub request_count: u64,
}

impl TokenUsageSnapshot {
    /// Sample the global [`super::super::super::TOKEN_USAGE`] counter right now.
    pub fn current() -> Self {
        let (prompt, comp, total) = super::super::super::TOKEN_USAGE.get();
        Self {
            name: "global".to_string(),
            prompt_tokens: prompt,
            completion_tokens: comp,
            total_tokens: total,
            totals: TokenTotals::new(prompt, comp),
            timestamp: Utc::now(),
            request_count: 0,
        }
    }

    /// Human-readable one-liner.
    pub fn summary(&self) -> String {
        format!(
            "{} total tokens ({} input, {} output)",
            self.totals.total(),
            self.prompt_tokens,
            self.completion_tokens
        )
    }
}
