//! Glm5PromptTokenDetails wire representation.
use crate::*;
#[derive(Debug, Deserialize, Default)]
pub(crate) struct Glm5PromptTokenDetails {
    #[serde(default)]
    pub(crate) cached_tokens: usize,
}
