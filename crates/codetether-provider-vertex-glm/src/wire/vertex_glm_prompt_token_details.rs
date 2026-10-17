//! Wire representation of VertexGlmPromptTokenDetails.
use crate::*;
#[derive(Debug, Deserialize, Default)]
pub(crate) struct VertexGlmPromptTokenDetails {
    #[serde(default)]
    pub(crate) cached_tokens: usize,
}
