//! Glm5Usage wire representation.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct Glm5Usage {
    #[serde(default)]
    pub(crate) prompt_tokens: usize,
    #[serde(default)]
    pub(crate) completion_tokens: usize,
    #[serde(default)]
    pub(crate) total_tokens: usize,
    /// OpenAI-compatible prompt-cache breakdown. GLM-5 surfaces
    /// `prompt_tokens_details.cached_tokens` when the implicit KV-cache
    /// hit on the input; we subtract it from `prompt_tokens` so the
    /// cost estimator can price the cached portion at the discounted
    /// rate (see the application pricing policy).
    #[serde(default)]
    pub(crate) prompt_tokens_details: Option<Glm5PromptTokenDetails>,
}
impl Glm5Usage {
    pub(crate) fn to_usage(&self) -> Usage {
        let cached = self
            .prompt_tokens_details
            .as_ref()
            .map(|d| d.cached_tokens)
            .unwrap_or(0);
        Usage {
            prompt_tokens: self.prompt_tokens.saturating_sub(cached),
            completion_tokens: self.completion_tokens,
            total_tokens: self.total_tokens,
            cache_read_tokens: if cached > 0 { Some(cached) } else { None },
            cache_write_tokens: None,
        }
    }
}
