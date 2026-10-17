//! CreateMessageResult wire contract and construction.
use super::*;

/// Sampling response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessageResult {
    pub role: PromptRole,
    pub content: SamplingContent,
    pub model: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,
}
