//! CallToolParams wire contract and construction.
use super::*;

/// Call tool parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallToolParams {
    pub name: String,
    #[serde(default)]
    pub arguments: Value,
}
