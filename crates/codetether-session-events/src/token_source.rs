//! Shared TokenSource event payload and operations.
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
/// use codetether_session_events::TokenSource;
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
    /// use codetether_session_events::TokenSource;
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
    /// use codetether_session_events::TokenSource;
    ///
    /// assert!(TokenSource::RlmIteration.is_rlm());
    /// assert!(TokenSource::RlmSubcall.is_rlm());
    /// assert!(!TokenSource::Root.is_rlm());
    /// ```
    pub const fn is_rlm(self) -> bool {
        matches!(self, Self::RlmIteration | Self::RlmSubcall)
    }
}
