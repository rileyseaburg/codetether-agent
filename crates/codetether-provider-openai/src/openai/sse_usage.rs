//! Standard and Groq usage metadata carried by streaming completion frames.
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct SseUsage {
    #[serde(default)]
    pub(super) prompt_tokens: usize,
    #[serde(default)]
    pub(super) completion_tokens: usize,
    #[serde(default)]
    pub(super) total_tokens: usize,
}

#[derive(Debug, Deserialize)]
pub(super) struct GroqMetadata {
    #[serde(default)]
    pub(super) usage: Option<SseUsage>,
}
