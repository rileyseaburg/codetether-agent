//! ReadResourceParams wire contract and construction.
use super::*;

/// Read resource parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadResourceParams {
    pub uri: String,
}
