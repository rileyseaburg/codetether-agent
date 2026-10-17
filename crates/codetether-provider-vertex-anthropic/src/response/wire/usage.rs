//! usage wire data.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct AnthropicUsage {
    #[serde(default)]
    pub(crate) input_tokens: usize,
    #[serde(default)]
    pub(crate) output_tokens: usize,
    #[serde(default)]
    pub(crate) cache_creation_input_tokens: Option<usize>,
    #[serde(default)]
    pub(crate) cache_read_input_tokens: Option<usize>,
}
