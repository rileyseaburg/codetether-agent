//! Wire schema for token usage in completions and SSE frames.
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct ZaiUsage {
    #[serde(default)]
    pub(crate) prompt_tokens: usize,
    #[serde(default)]
    pub(crate) completion_tokens: usize,
    #[serde(default)]
    pub(crate) total_tokens: usize,
    #[serde(default)]
    pub(crate) prompt_tokens_details: Option<ZaiPromptTokensDetails>,
}
#[derive(Debug, Deserialize)]
pub(crate) struct ZaiPromptTokensDetails {
    #[serde(default)]
    pub(crate) cached_tokens: usize,
}
