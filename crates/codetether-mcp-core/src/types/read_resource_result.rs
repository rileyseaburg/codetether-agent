//! ReadResourceResult wire contract and construction.
use super::*;

/// Read resource response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadResourceResult {
    pub contents: Vec<ResourceContents>,
}
