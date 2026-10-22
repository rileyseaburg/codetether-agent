//! Streaming event variants shared by native session consumers.
use crate::{
    CompactionFailure, CompactionOutcome, CompactionStart, ContextTruncation, RlmCompletion,
    RlmProgressEvent, RlmSubcallFallback, StreamRetryEvent, TokenDelta, TokenEstimate,
};

/// Events emitted during session processing.
///
/// `S` is the application-owned snapshot carried by `SessionSync`. Use `()`
/// for consumers that never attach a session snapshot.
///
/// # Examples
///
/// ```
/// use codetether_session_events::SessionEvent;
///
/// let event = SessionEvent::<()>::Thinking;
/// assert!(!event.is_durable());
/// ```
#[derive(Debug, Clone)]
#[rustfmt::skip]
#[non_exhaustive]
pub enum SessionEvent<S = ()> {
    /// The agent is thinking or waiting on the model.
    Thinking,
    /// Synthetic provider-wait liveness tick; no model output was received.
    /// Consumers may refresh watchdogs but must not treat this as task activity.
    ProviderKeepalive,
    /// Opens or closes a lightweight activity while work continues.
    PlayBreak(bool),
    /// A tool call has started.
    ToolCallStart { tool_call_id: String, name: String, arguments: String },
    /// A tool call has completed.
    ToolCallComplete { tool_call_id: String, name: String, output: String, success: bool, duration_ms: u64 },
    /// A running tool reports elapsed time.
    ToolHeartbeat { tool_call_id: String, name: String, elapsed_secs: u64 },
    /// Incremental output from a running tool stream.
    ToolOutputChunk { tool_call_id: String, name: String, stream: String, chunk: String },
    /// Structured metadata produced by an in-flight tool call.
    ToolCallMetadata { tool_call_id: String, name: String, metadata: serde_json::Value },
    /// A tool call is paused until the UI approves or denies it.
    ApprovalRequest(codetether_approval_core::interaction::LiveApprovalRequest),
    /// Partial assistant text output.
    TextChunk(String),
    /// A failed stream attempt is being replaced by a fresh request.
    StreamRetry(StreamRetryEvent),
    /// Final per-step assistant text output.
    TextComplete(String),
    /// Model thinking or reasoning output.
    ThinkingComplete(String),
    /// Legacy aggregate token usage and timing.
    UsageReport { prompt_tokens: usize, completion_tokens: usize, duration_ms: u64, model: String },
    /// Updated session state for caller synchronization.
    SessionSync(Box<S>),
    /// Processing is complete.
    Done,
    /// Processing failed.
    Error(String),
    /// Pre-flight estimate of the next request's token footprint.
    TokenEstimate(TokenEstimate),
    /// Observed token consumption for one LLM round-trip.
    TokenUsage(TokenDelta),
    /// Per-iteration progress tick from an in-flight RLM loop.
    RlmProgress(RlmProgressEvent),
    /// Terminal record for an RLM invocation.
    RlmComplete(RlmCompletion),
    /// A context-compaction pass has begun.
    CompactionStarted(CompactionStart),
    /// A context-compaction pass has finished successfully.
    CompactionCompleted(CompactionOutcome),
    /// Every compaction strategy failed to fit under budget.
    CompactionFailed(CompactionFailure),
    /// The terminal truncation fallback dropped part of the transcript.
    ContextTruncated(ContextTruncation),
    /// A configured `subcall_model` could not be resolved.
    RlmSubcallFallback(RlmSubcallFallback),
}
