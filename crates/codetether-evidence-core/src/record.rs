//! Serializable evidence observations extracted from text.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Serializable marker extracted from reported evidence text.
/// # Examples
/// ```
/// let record = codetether_evidence_core::record::EvidenceRecord::new("artifact", "static/local", "a.zip");
/// assert_eq!(record.value, "a.zip");
/// ```
pub struct EvidenceRecord {
    /// Evidence source category.
    pub kind: String,
    /// Reported evidence level.
    pub level: String,
    /// Extracted identifier, path or state marker.
    pub value: String,
}

impl EvidenceRecord {
    /// Construct an owned evidence observation from its category, level and value.
    pub fn new(kind: &str, level: &str, value: impl Into<String>) -> Self {
        Self {
            kind: kind.to_string(),
            level: level.to_string(),
            value: value.into(),
        }
    }
}
