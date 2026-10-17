//! usage wire representation.
use serde::Deserialize;
use super::*;
#[derive(Debug, Deserialize)]
pub(crate) struct DsUsage {
    #[serde(default)]
    pub prompt_tokens: usize,
    #[serde(default)]
    pub completion_tokens: usize,
    #[serde(default)]
    pub total_tokens: usize,
}
