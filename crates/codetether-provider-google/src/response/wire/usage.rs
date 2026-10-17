//! usage response schema.
use super::*;
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct ApiUsage {
    #[serde(default)]
    pub(crate) prompt_tokens: usize,
    #[serde(default)]
    pub(crate) completion_tokens: usize,
    #[serde(default)]
    pub(crate) total_tokens: usize,
    /// Gemini `usageMetadata.cachedContentTokenCount` exposed via the
    /// OpenAI-compat shim. Counted within `prompt_tokens`; we subtract
    /// it when populating [`Usage`] so the cost estimator prices it at
    /// the discounted rate (the provider cost estimator applies the cache-read discount).
    #[serde(default, rename = "cached_tokens")]
    pub(crate) cached_tokens: Option<usize>,
    #[serde(default, rename = "prompt_tokens_details")]
    pub(crate) prompt_tokens_details: Option<PromptTokenDetails>,
}
#[derive(Debug, Deserialize, Default)]
pub(crate) struct PromptTokenDetails {
    #[serde(default)]
    pub(crate) cached_tokens: usize,
}
impl ApiUsage {
    pub(crate) fn cached_input_tokens(&self) -> usize {
        self.cached_tokens.unwrap_or_else(|| {
            self.prompt_tokens_details
                .as_ref()
                .map(|d| d.cached_tokens)
                .unwrap_or(0)
        })
    }
}
