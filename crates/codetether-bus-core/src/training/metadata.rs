//! Training record data used by the serialization pipeline.
use super::*;

/// Provenance metadata attached to every training record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct TrainingMetadata {
    /// Original BusMessage variant name (snake_case)
    pub(super) bus_kind: String,
    /// Envelope id
    pub(super) envelope_id: String,
    /// ISO 8601 timestamp
    pub(super) timestamp: String,
    /// Hierarchical topic
    pub(super) topic: String,
    /// Agent that originated the message
    pub(super) sender_id: String,
    /// Correlation id linking request ↔ response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) correlation_id: Option<String>,
    /// Agent loop step number when available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) step: Option<usize>,
}
