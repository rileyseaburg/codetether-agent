//! ToolMetadata wire contract and construction.
use super::*;

/// Tool metadata for storage and querying
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolMetadata {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: Value,
    /// Optional endpoint for the tool (e.g., HTTP URL)
    pub endpoint: Option<String>,
    /// Additional metadata about the tool
    pub metadata: Option<Value>,
    pub registered_at: std::time::SystemTime,
}

impl ToolMetadata {
    pub fn new(name: String, description: Option<String>, input_schema: Value) -> Self {
        Self {
            name,
            description,
            input_schema,
            endpoint: None,
            metadata: None,
            registered_at: std::time::SystemTime::now(),
        }
    }

    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    pub fn with_metadata(mut self, metadata: Value) -> Self {
        self.metadata = Some(metadata);
        self
    }
}
