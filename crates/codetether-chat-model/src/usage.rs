//! Provider usage attributed to a displayed message.
use serde::{Deserialize, Serialize};

/// Token usage attributed to a single chat message.
///
/// Populated from a session usage-report event after a
/// provider completion returns. Attached to the most recent assistant /
/// tool-call message produced by that completion.
///
/// # Examples
///
/// ```rust
/// use codetether_chat_model::MessageUsage;
///
/// let usage = MessageUsage {
///     model: "anthropic/claude-opus-4".into(),
///     prompt_tokens: 1523,
///     completion_tokens: 284,
///     duration_ms: 4210,
/// };
/// assert_eq!(usage.prompt_tokens, 1523);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MessageUsage {
    /// Model that produced this completion.
    pub model: String,
    /// Input / prompt tokens charged for this completion.
    pub prompt_tokens: usize,
    /// Output / completion tokens produced by this completion.
    pub completion_tokens: usize,
    /// Wall-clock latency of the provider request in milliseconds.
    pub duration_ms: u64,
}
