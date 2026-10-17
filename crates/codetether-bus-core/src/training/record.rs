//! Training record data used by the serialization pipeline.
use super::*;

// ─── LLM Pretraining Record ─────────────────────────────────────────────

/// One chronological chat message in the training event stream.
///
/// Dataset loaders group these records by correlation id to build complete
/// chat trajectories while retaining source provenance for quality filters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct TrainingRecord {
    /// "system" | "user" | "assistant" | "tool"
    pub(super) role: String,
    /// Primary text content
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) content: Option<String>,
    /// Tool calls made by the assistant (only when role == "assistant")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) tool_calls: Option<Vec<TrainingToolCall>>,
    /// Tool call id this result corresponds to (only when role == "tool")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) tool_call_id: Option<String>,
    /// Tool name (only when role == "tool")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) name: Option<String>,
    /// Envelope metadata for filtering and curriculum design
    pub(super) metadata: TrainingMetadata,
}
