//! Wire representation of ApiUsage.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct ApiUsage {
    #[serde(default)]
    pub(crate) prompt_tokens: usize,
    #[serde(default)]
    pub(crate) completion_tokens: usize,
    #[serde(default)]
    pub(crate) total_tokens: usize,
    /// GLM-on-Vertex KV-cache hit count, subset of `prompt_tokens`.
    #[serde(default)]
    pub(crate) prompt_tokens_details: Option<VertexGlmPromptTokenDetails>,
}
impl ApiUsage {
    pub(crate) fn cached(&self) -> usize {
        self.prompt_tokens_details
            .as_ref()
            .map(|d| d.cached_tokens)
            .unwrap_or(0)
    }
}
