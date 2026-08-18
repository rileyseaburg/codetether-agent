//! Completion result contract for thinker backends.

/// Output from a thinker completion.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::cognition::ThinkerOutput;
/// let out = ThinkerOutput {
///     model: "qwen3.5-9b".into(),
///     finish_reason: Some("stop".into()),
///     text: "hello".into(),
///     prompt_tokens: Some(5),
///     completion_tokens: Some(1),
///     total_tokens: Some(6),
///     cache_read_tokens: None,
///     cache_write_tokens: None,
/// };
/// assert_eq!(out.model, "qwen3.5-9b");
/// ```
#[derive(Debug, Clone)]
pub struct ThinkerOutput {
    pub model: String,
    pub finish_reason: Option<String>,
    pub text: String,
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
    pub cache_read_tokens: Option<u32>,
    pub cache_write_tokens: Option<u32>,
}
