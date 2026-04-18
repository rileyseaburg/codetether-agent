//! Event and result types emitted by session prompt methods.
//!
//! [`SessionResult`] is the terminal value returned from a successful
//! prompt; [`SessionEvent`] is streamed to a UI channel for real-time
//! feedback (tool calls, partial text, usage, compaction, etc.).
//!
//! # Ephemeral vs durable events
//!
//! [`SessionEvent`] is `#[non_exhaustive]` and carries two classes of
//! payload:
//!
//! - **Ephemeral** — safe to drop under load. Used by the TUI for status,
//!   spinners, and token badges. Examples: [`SessionEvent::Thinking`],
//!   [`SessionEvent::TextChunk`], [`SessionEvent::RlmProgress`],
//!   [`SessionEvent::TokenEstimate`].
//! - **Durable** — must reach the JSONL flywheel for cost post-mortems
//!   and trace-driven tuning. Examples: [`SessionEvent::TokenUsage`],
//!   [`SessionEvent::RlmComplete`], [`SessionEvent::CompactionCompleted`],
//!   [`SessionEvent::CompactionFailed`], [`SessionEvent::ContextTruncated`].
//!
//! [`SessionEvent::is_durable`] lets consumers route accordingly. The
//! unified [`SessionBus`](crate::session::SessionBus) uses this to
//! dispatch ephemeral events via a lossy `tokio::sync::broadcast` channel
//! while forwarding durable events through a write-ahead
//! [`DurableSink`](crate::session::DurableSink).

use serde::{Deserialize, Serialize};

use super::event_compaction::{
    CompactionFailure, CompactionOutcome, CompactionStart, ContextTruncation,
};
use super::event_rlm::{RlmCompletion, RlmProgressEvent, RlmSubcallFallback};
use super::event_token::{TokenDelta, TokenEstimate};
use super::types::Session;

/// Result returned from [`Session::prompt`](crate::session::Session::prompt)
/// and friends.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResult {
    /// Final assistant text answer (trimmed).
    pub text: String,
    /// UUID of the session that produced the answer.
    pub session_id: String,
}

/// Events emitted during session processing for real-time UI updates and
/// durable telemetry.
///
/// # Stability
///
/// This enum is `#[non_exhaustive]`. `match` expressions must include a
/// wildcard arm; new variants may be added without breaking consumers.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::SessionEvent;
///
/// let ev = SessionEvent::Thinking;
/// assert!(!ev.is_durable());
/// assert!(matches!(ev, SessionEvent::Thinking));
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SessionEvent {
    /// The agent is thinking / waiting on the model.
    Thinking,
    /// A tool call has started.
    ToolCallStart {
        /// Tool name.
        name: String,
        /// Tool arguments (JSON-encoded).
        arguments: String,
    },
    /// A tool call has completed.
    ToolCallComplete {
        /// Tool name.
        name: String,
        /// Rendered tool output.
        output: String,
        /// Whether the tool reported success.
        success: bool,
        /// End-to-end execution duration in milliseconds.
        duration_ms: u64,
    },
    /// Partial assistant text output for streaming UIs.
    TextChunk(String),
    /// Final (per-step) assistant text output.
    TextComplete(String),
    /// Model thinking/reasoning output (for reasoning-capable models).
    ThinkingComplete(String),
    /// Token usage and timing for one LLM round-trip (legacy aggregate).
    ///
    /// Prefer [`SessionEvent::TokenUsage`] for new code — it carries a
    /// [`TokenSource`](crate::session::TokenSource) so RLM sub-calls and
    /// tool-embedded LLM calls are attributed separately.
    UsageReport {
        /// Prompt tokens consumed.
        prompt_tokens: usize,
        /// Completion tokens produced.
        completion_tokens: usize,
        /// Round-trip duration in milliseconds.
        duration_ms: u64,
        /// Model ID that served the request.
        model: String,
    },
    /// Updated session state so the caller can sync its in-memory copy.
    SessionSync(Box<Session>),
    /// Processing is complete.
    Done,
    /// An error occurred during processing.
    Error(String),
    /// Pre-flight estimate of the next request's token footprint.
    /// Ephemeral.
    TokenEstimate(TokenEstimate),
    /// Observed token consumption for one LLM round-trip, attributed by
    /// [`TokenSource`](crate::session::TokenSource). Durable.
    TokenUsage(TokenDelta),
    /// Per-iteration progress tick from an in-flight RLM loop. Ephemeral.
    RlmProgress(RlmProgressEvent),
    /// Terminal record for an RLM invocation. Durable.
    RlmComplete(RlmCompletion),
    /// A context-compaction pass has begun. Durable.
    CompactionStarted(CompactionStart),
    /// A context-compaction pass has finished successfully. Durable.
    CompactionCompleted(CompactionOutcome),
    /// Every compaction strategy failed to fit under budget. Durable.
    CompactionFailed(CompactionFailure),
    /// The terminal truncation fallback dropped part of the transcript.
    /// Durable; emitted in addition to [`SessionEvent::CompactionCompleted`]
    /// when the final strategy is
    /// [`FallbackStrategy::Truncate`](crate::session::FallbackStrategy::Truncate).
    ContextTruncated(ContextTruncation),
    /// A configured `subcall_model` could not be resolved so the router
    /// fell back to the root model. Durable — this is a cost signal.
    RlmSubcallFallback(RlmSubcallFallback),
}

impl SessionEvent {
    /// Returns `true` if this variant carries data that must reach the
    /// durable sink (see the module docs for the full split).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::session::{SessionEvent, TokenDelta, TokenSource};
    ///
    /// let delta = TokenDelta {
    ///     source: TokenSource::Root,
    ///     model: "m".into(),
    ///     prompt_tokens: 1, completion_tokens: 1, duration_ms: 0,
    /// };
    /// assert!(SessionEvent::TokenUsage(delta).is_durable());
    /// assert!(!SessionEvent::Thinking.is_durable());
    /// assert!(!SessionEvent::TextChunk("x".into()).is_durable());
    /// ```
    pub fn is_durable(&self) -> bool {
        matches!(
            self,
            Self::TokenUsage(_)
                | Self::RlmComplete(_)
                | Self::CompactionStarted(_)
                | Self::CompactionCompleted(_)
                | Self::CompactionFailed(_)
                | Self::ContextTruncated(_)
                | Self::RlmSubcallFallback(_)
        )
    }
}
