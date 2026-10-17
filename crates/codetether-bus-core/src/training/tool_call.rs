//! Training record data used by the serialization pipeline.
use super::*;

/// Represents a tool call in the assistant's response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct TrainingToolCall {
    /// Unique id for this tool call
    pub(super) id: String,
    /// Always "function"
    #[serde(rename = "type")]
    pub(super) call_type: String,
    /// The function being called
    pub(super) function: TrainingFunction,
}
