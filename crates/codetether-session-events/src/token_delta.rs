//! Shared TokenDelta event payload and operations.
use crate::TokenSource;
use serde::{Deserialize, Serialize};

/// A single token-consumption delta observed during session processing.
///
/// One `TokenDelta` is emitted per LLM round-trip (root or RLM), allowing
/// downstream consumers to maintain per-model and per-source counters
/// without re-reading the full session history.
///
/// # Examples
///
/// ```rust
/// use codetether_session_events::{TokenDelta, TokenSource};
///
/// let delta = TokenDelta {
///     source: TokenSource::Root,
///     model: "gpt-4o".to_string(),
///     prompt_tokens: 1_200,
///     completion_tokens: 340,
///     duration_ms: 820,
/// };
/// assert_eq!(delta.total(), 1_540);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenDelta {
    /// What kind of call produced this delta.
    pub source: TokenSource,
    /// Provider-qualified model identifier (e.g. `"anthropic/claude-opus-4-7"`).
    pub model: String,
    /// Prompt tokens consumed.
    pub prompt_tokens: usize,
    /// Completion tokens produced.
    pub completion_tokens: usize,
    /// Wall-clock round-trip in milliseconds.
    pub duration_ms: u64,
}

impl TokenDelta {
    /// Sum of prompt and completion tokens.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_session_events::{TokenDelta, TokenSource};
    ///
    /// let d = TokenDelta {
    ///     source: TokenSource::Root,
    ///     model: "m".into(),
    ///     prompt_tokens: 10,
    ///     completion_tokens: 5,
    ///     duration_ms: 0,
    /// };
    /// assert_eq!(d.total(), 15);
    /// ```
    pub fn total(&self) -> usize {
        self.prompt_tokens.saturating_add(self.completion_tokens)
    }
}
