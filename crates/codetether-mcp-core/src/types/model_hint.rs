//! ModelHint wire contract and construction.
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelHint {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
