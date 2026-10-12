use serde::{Deserialize, Serialize};

/// Token usage statistics, including optional prompt-cache usage.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::Usage;
/// let usage = Usage { prompt_tokens: 10, completion_tokens: 5, ..Usage::default() };
/// assert_eq!(usage.total_tokens, 0); // Not computed automatically.
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Usage {
    /// Tokens in the prompt.
    pub prompt_tokens: usize,
    /// Tokens in the completion.
    pub completion_tokens: usize,
    /// Total (prompt + completion).
    pub total_tokens: usize,
    /// Prompt-cache read hits (Anthropic / Bedrock).
    pub cache_read_tokens: Option<usize>,
    /// Prompt-cache write misses.
    pub cache_write_tokens: Option<usize>,
}
