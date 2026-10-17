//! ResourceMetadata wire contract and construction.
use super::*;

/// Resource metadata for storage and querying
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetadata {
    pub uri: String,
    pub name: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
    pub registered_at: std::time::SystemTime,
}

impl ResourceMetadata {
    pub fn new(
        uri: String,
        name: String,
        description: Option<String>,
        mime_type: Option<String>,
    ) -> Self {
        Self {
            uri,
            name,
            description,
            mime_type,
            registered_at: std::time::SystemTime::now(),
        }
    }
}
