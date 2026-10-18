//! Codex usage record payload schemas.
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct CodexTokenEnvelope {
    pub(crate) info: Option<CodexTokenInfo>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CodexTokenInfo {
    pub(crate) total_token_usage: CodexTotalTokenUsage,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CodexTotalTokenUsage {
    #[serde(default)]
    pub(crate) input_tokens: usize,
    #[serde(default)]
    pub(crate) cached_input_tokens: Option<usize>,
    #[serde(default)]
    pub(crate) output_tokens: usize,
    #[serde(default)]
    pub(crate) total_tokens: usize,
}
