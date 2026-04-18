//! Event and result types emitted by session prompt methods.
//!
//! [`SessionResult`] is the terminal value returned from a successful
//! prompt; [`SessionEvent`] is streamed to a UI channel for real-time
//! feedback (tool calls, partial text, usage, etc.).

use serde::{Deserialize, Serialize};

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

/// Events emitted during session processing for real-time UI updates.
#[derive(Debug, Clone)]
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
    /// Token usage and timing for one LLM round-trip.
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
}
