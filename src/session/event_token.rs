//! Token-accounting payloads carried by [`SessionEvent`].
//!
//! These types back the [`SessionEvent::TokenUsage`] and
//! [`SessionEvent::TokenEstimate`] variants. They are intentionally
//! newtype-wrapped so their wire format is stable independently of the
//! enum itself — the durable JSONL sink consumes them directly, and the
//! TUI status line renders [`TokenEstimate`] against the current model
//! budget on every request.
//!
//! [`SessionEvent`]: crate::session::SessionEvent
//! [`SessionEvent::TokenUsage`]: crate::session::SessionEvent::TokenUsage
//! [`SessionEvent::TokenEstimate`]: crate::session::SessionEvent::TokenEstimate

use serde::{Deserialize, Serialize};

/// Origin of a token accounting delta.
///
/// Distinguishes the root chat round-trip from tokens spent inside
/// Recursive Language Model (RLM) sub-processing so cost dashboards can
/// break down spend by source instead of aggregating everything under the
/// chat model.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::TokenSource;
///
/// let src = TokenSource::RlmSubcall;
/// assert_eq!(src.as_str(), "rlm_subcall");
/// assert!(src.is_rlm());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenSource {
    /// A primary chat completion for the user-facing session.
    Root,
    /// A single iteration of the RLM analysis loop (root RLM model).
    RlmIteration,
    /// A sub-LLM call dispatched from inside the RLM loop (subcall model).
    RlmSubcall,
    /// An LLM call issued from inside a tool implementation.
    ToolEmbedded,
}

impl TokenSource {
    /// Stable string identifier suitable for logs and JSONL records.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::session::TokenSource;
    ///
    /// assert_eq!(TokenSource::Root.as_str(), "root");
    /// assert_eq!(TokenSource::ToolEmbedded.as_str(), "tool_embedded");
    /// ```
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Root => "root",
            Self::RlmIteration => "rlm_iteration",
            Self::RlmSubcall => "rlm_subcall",
            Self::ToolEmbedded => "tool_embedded",
        }
    }

    /// Returns `true` when the source is any RLM-attributable variant.
    ///
    /// Useful for telemetry splits like "chat tokens vs RLM overhead".
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::session::TokenSource;
    ///
    /// assert!(TokenSource::RlmIteration.is_rlm());
    /// assert!(TokenSource::RlmSubcall.is_rlm());
    /// assert!(!TokenSource::Root.is_rlm());
    /// ```
    pub const fn is_rlm(self) -> bool {
        matches!(self, Self::RlmIteration | Self::RlmSubcall)
    }
}

/// A single token-consumption delta observed during session processing.
///
/// One `TokenDelta` is emitted per LLM round-trip (root or RLM), allowing
/// downstream consumers to maintain per-model and per-source counters
/// without re-reading the full session history.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::{TokenDelta, TokenSource};
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
    /// use codetether_agent::session::{TokenDelta, TokenSource};
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
/// use codetether_agent::session::TokenEstimate;
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
    /// use codetether_agent::session::TokenEstimate;
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
    /// use codetether_agent::session::TokenEstimate;
    ///
    /// let e = TokenEstimate { model: "m".into(), request_tokens: 129_000, budget: 128_000 };
    /// assert!(e.is_over_budget());
    /// ```
    pub fn is_over_budget(&self) -> bool {
        self.request_tokens > self.budget
    }
}
