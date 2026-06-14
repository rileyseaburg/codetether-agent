//! Streaming session event variants.

use super::super::event_compaction::{
    CompactionFailure, CompactionOutcome, CompactionStart, ContextTruncation,
};
use super::super::event_rlm::{RlmCompletion, RlmProgressEvent, RlmSubcallFallback};
use super::super::event_token::{TokenDelta, TokenEstimate};
use super::super::types::Session;

/// Events emitted during session processing.
///
/// # Examples
///
/// ```
/// use codetether_agent::session::SessionEvent;
///
/// let event = SessionEvent::Thinking;
/// assert!(!event.is_durable());
/// ```
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SessionEvent {
    /// The agent is thinking or waiting on the model.
    Thinking,
    /// A tool call has started.
    ToolCallStart { tool_call_id: String, name: String, arguments: String },
    /// A tool call has completed.
    ToolCallComplete {
        tool_call_id: String,
        name: String,
        output: String,
        success: bool,
        duration_ms: u64,
    },
    ToolHeartbeat { tool_call_id: String, name: String, elapsed_secs: u64 },
    /// Structured metadata produced by an in-flight tool call.
    ToolCallMetadata {
        tool_call_id: String,
        name: String,
        metadata: serde_json::Value,
    },
    /// A tool call is paused until the UI approves or denies it.
    ApprovalRequest(crate::approval::LiveApprovalRequest),
    /// Partial assistant text output.
    TextChunk(String),
    /// Final per-step assistant text output.
    TextComplete(String),
    /// Model thinking or reasoning output.
    ThinkingComplete(String),
    /// Legacy aggregate token usage and timing.
    UsageReport {
        prompt_tokens: usize,
        completion_tokens: usize,
        duration_ms: u64,
        model: String,
    },
    /// Updated session state for caller synchronization.
    SessionSync(Box<Session>),
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
