//! Token usage wire schema.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct MoonshotUsage {
    #[serde(default)]
    pub(crate) prompt_tokens: usize,
    #[serde(default)]
    pub(crate) completion_tokens: usize,
    #[serde(default)]
    pub(crate) total_tokens: usize,
}
