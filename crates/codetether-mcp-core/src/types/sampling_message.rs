//! SamplingMessage wire contract and construction.
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SamplingMessage {
    pub role: PromptRole,
    pub content: SamplingContent,
}
