//! ConverseUsage wire shape.
use super::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ConverseUsage {
    #[serde(default)]
    pub(crate) input_tokens: usize,
    #[serde(default)]
    pub(crate) output_tokens: usize,
    #[serde(default)]
    pub(crate) total_tokens: usize,
}
